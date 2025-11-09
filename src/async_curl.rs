//! Async curl implementation using tokio

use crate::curl::Curl;
use crate::error::{check_multi_code, CurlError, Result};
use futures::future::BoxFuture;
use std::collections::HashMap;
use std::os::raw::c_int;
use std::ptr;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

/// Async Curl multi handle wrapper
pub struct AsyncCurl {
    multi: *mut curl_sys::CURLM,
    handles: Arc<Mutex<HashMap<usize, CurlHandle>>>,
}

struct CurlHandle {
    curl: *mut curl_sys::CURL,
    sender: oneshot::Sender<Result<()>>,
    buffer: Vec<u8>,
}

impl AsyncCurl {
    /// Create a new AsyncCurl instance
    pub fn new() -> Result<Self> {
        let multi = unsafe { curl_sys::curl_multi_init() };
        if multi.is_null() {
            return Err(CurlError::InitError);
        }

        Ok(AsyncCurl {
            multi,
            handles: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Perform an async request
    pub async fn perform(&self, mut curl: Curl) -> Result<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        let mut buffer = Vec::new();

        // Add to multi handle
        unsafe {
            let code = curl_sys::curl_multi_add_handle(self.multi, curl.raw_handle());
            check_multi_code(code)?;
        }

        let handle_id = curl.raw_handle() as usize;
        {
            let mut handles = self.handles.lock().unwrap();
            handles.insert(
                handle_id,
                CurlHandle {
                    curl: curl.raw_handle(),
                    sender: tx,
                    buffer,
                },
            );
        }

        // Process in background (simplified - real impl would use socket_action)
        let multi = self.multi;
        let handles = self.handles.clone();

        tokio::task::spawn_blocking(move || {
            unsafe {
                let mut running: c_int = 0;
                loop {
                    curl_sys::curl_multi_perform(multi, &mut running);
                    if running == 0 {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }

                // Check for completed transfers
                let mut msgs_in_queue: c_int = 0;
                loop {
                    let msg = curl_sys::curl_multi_info_read(multi, &mut msgs_in_queue);
                    if msg.is_null() {
                        break;
                    }

                    let msg_ref = &*msg;
                    if msg_ref.msg == curl_sys::CURLMSG_DONE {
                        let handle_id = msg_ref.easy_handle as usize;
                        let result = if msg_ref.data.result == curl_sys::CURLE_OK {
                            Ok(())
                        } else {
                            Err(CurlError::from_curl_code(msg_ref.data.result))
                        };

                        if let Some(handle) = handles.lock().unwrap().remove(&handle_id) {
                            let _ = handle.sender.send(result);
                        }

                        curl_sys::curl_multi_remove_handle(multi, msg_ref.easy_handle);
                    }
                }
            }
        });

        // Wait for completion
        match rx.await {
            Ok(result) => {
                result?;
                // Return buffer (simplified)
                Ok(Vec::new())
            }
            Err(_) => Err(CurlError::Other("Channel closed".to_string())),
        }
    }

    /// Perform multiple requests concurrently
    pub async fn perform_many(&self, curls: Vec<Curl>) -> Vec<Result<Vec<u8>>> {
        let mut futures = Vec::new();
        for curl in curls {
            futures.push(self.perform(curl));
        }

        futures::future::join_all(futures).await
    }
}

impl Drop for AsyncCurl {
    fn drop(&mut self) {
        if !self.multi.is_null() {
            unsafe {
                curl_sys::curl_multi_cleanup(self.multi);
            }
        }
    }
}

unsafe impl Send for AsyncCurl {}
unsafe impl Sync for AsyncCurl {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_curl_new() {
        let async_curl = AsyncCurl::new();
        assert!(async_curl.is_ok());
    }
}
