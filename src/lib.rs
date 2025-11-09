//! # curl-cffi-rs
//!
//! A high-performance Rust implementation of curl_cffi with browser fingerprinting support.
//!
//! This crate provides a safe, ergonomic wrapper around libcurl-impersonate with:
//! - Zero-cost abstractions
//! - Type-safe API
//! - Async/await support (via tokio)
//! - WebSocket support
//! - Python bindings (via PyO3)
//!
//! ## Example
//!
//! ```no_run
//! use curl_cffi_rs::{Curl, CurlError};
//!
//! fn main() -> Result<(), CurlError> {
//!     let mut curl = Curl::new()?;
//!     curl.set_url("https://httpbin.org/get")?;
//!
//!     let mut response = Vec::new();
//!     curl.perform(&mut response)?;
//!
//!     println!("Response: {}", String::from_utf8_lossy(&response));
//!     Ok(())
//! }
//! ```

pub mod curl;
pub mod error;
pub mod types;
pub mod websocket;

#[cfg(feature = "async")]
pub mod async_curl;

#[cfg(feature = "python")]
pub mod python;

// Re-exports
pub use curl::Curl;
pub use error::{CurlError, Result};
pub use types::{CurlInfo, CurlOpt, HttpVersion};
pub use websocket::WebSocket;

#[cfg(feature = "async")]
pub use async_curl::AsyncCurl;

// Version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Initialize curl globally once
use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize libcurl globally (called automatically)
pub(crate) fn init_curl() {
    INIT.call_once(|| {
        unsafe {
            curl_sys::curl_global_init(curl_sys::CURL_GLOBAL_DEFAULT);
        }
    });
}

/// Get libcurl version information
pub fn version() -> String {
    init_curl();
    unsafe {
        let version_ptr = curl_sys::curl_version();
        std::ffi::CStr::from_ptr(version_ptr)
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let ver = version();
        assert!(ver.contains("curl") || ver.contains("libcurl"));
    }
}
