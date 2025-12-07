//! Test with safer callback approach

use curl_cffi_rs::Curl;
use std::ffi::c_void;
use std::os::raw::c_char;

// Simple write callback that just counts bytes
unsafe extern "C" fn count_callback(
    _ptr: *mut c_char,
    size: usize,
    nmemb: usize,
    userdata: *mut c_void,
) -> usize {
    let total = size * nmemb;
    let counter = &mut *(userdata as *mut usize);
    *counter += total;
    total  // MUST return total bytes to indicate success
}

#[test]
fn test_with_counting_callback() {
    eprintln!("Creating Curl...");
    let mut curl = Curl::new().expect("Failed to create Curl");

    eprintln!("Setting URL...");
    curl.set_url("http://example.com").expect("Failed to set URL");

    // Use a simple counter instead of Vec<u8>
    let mut byte_count: usize = 0;

    eprintln!("Setting callback...");
    unsafe {
        curl_sys::curl_easy_setopt(
            curl.raw_handle(),
            curl_sys::CURLOPT_WRITEFUNCTION,
            count_callback as *const c_void,
        );

        curl_sys::curl_easy_setopt(
            curl.raw_handle(),
            curl_sys::CURLOPT_WRITEDATA,
            &mut byte_count as *mut usize as *mut c_void,
        );
    }

    eprintln!("Performing request...");
    unsafe {
        let code = curl_sys::curl_easy_perform(curl.raw_handle());
        assert_eq!(code, 0, "Perform should succeed");
    }

    eprintln!("Bytes received: {}", byte_count);
    assert!(byte_count > 0, "Should have received some bytes");

    println!("✅ Callback test passed! Received {} bytes", byte_count);
}
