//! Async curl implementation using tokio

use crate::curl::Curl;
use crate::error::{CurlError, Result};
use std::sync::Arc;
use tokio::sync::oneshot;

/// Async Curl wrapper using tokio task spawning
pub struct AsyncCurl {
    _private: (),
}

impl AsyncCurl {
    /// Create a new AsyncCurl instance
    pub fn new() -> Result<Self> {
        Ok(AsyncCurl { _private: () })
    }

    /// Perform an async request by running sync curl in a blocking task
    pub async fn perform(&self, mut curl: Curl) -> Result<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        
        // Spawn the blocking curl operation in a separate thread
        tokio::task::spawn_blocking(move || {
            let mut response = Vec::new();
            let result = curl.perform(&mut response);
            
            // Send result back
            let _ = tx.send((result, response));
        });

        // Wait for completion
        match rx.await {
            Ok((Ok(_), response)) => Ok(response),
            Ok((Err(e), _)) => Err(e),
            Err(_) => Err(CurlError::Other("Task cancelled".to_string())),
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

    #[tokio::test]
    async fn test_async_perform() {
        let async_curl = AsyncCurl::new().unwrap();
        let mut curl = Curl::new().unwrap();
        curl.set_url("http://httpbin.org/get").unwrap();
        
        let result = async_curl.perform(curl).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(!response.is_empty());
    }
}