//! Test with Vec callback using Box to prevent pointer invalidation

use curl_cffi_rs::Curl;
use std::ffi::c_void;
use std::os::raw::c_char;

// Write callback that appends to Vec<u8>
unsafe extern "C" fn vec_write_callback(
    ptr: *mut c_char,
    size: usize,
    nmemb: usize,
    userdata: *mut c_void,
) -> usize {
    let total_size = size * nmemb;

    if total_size == 0 {
        return 0;
    }

    // Get Box<Vec<u8>> from userdata
    let buffer_box = &mut *(userdata as *mut Box<Vec<u8>>);
    let buffer = buffer_box.as_mut();

    // Create slice and copy data
    let data = std::slice::from_raw_parts(ptr as *const u8, total_size);
    buffer.extend_from_slice(data);

    total_size
}

#[test]
fn test_vec_with_box() {
    eprintln!("Creating Curl...");
    let mut curl = Curl::new().expect("Failed to create Curl");

    eprintln!("Setting URL...");
    curl.set_url("http://example.com").expect("Failed to set URL");

    // Use Box to keep Vec on heap with stable address
    let mut buffer_box = Box::new(Vec::new());

    eprintln!("Setting callback...");
    unsafe {
        curl_sys::curl_easy_setopt(
            curl.raw_handle(),
            curl_sys::CURLOPT_WRITEFUNCTION,
            vec_write_callback as *const c_void,
        );

        // Pass pointer to Box (which won't move)
        curl_sys::curl_easy_setopt(
            curl.raw_handle(),
            curl_sys::CURLOPT_WRITEDATA,
            &mut buffer_box as *mut Box<Vec<u8>> as *mut c_void,
        );
    }

    eprintln!("Performing request...");
    unsafe {
        let code = curl_sys::curl_easy_perform(curl.raw_handle());
        assert_eq!(code, 0, "Perform should succeed");
    }

    let response = String::from_utf8_lossy(&buffer_box);
    eprintln!("Response length: {} bytes", buffer_box.len());
    eprintln!("First 100 chars: {}", &response[..100.min(response.len())]);

    assert!(buffer_box.len() > 0, "Should have received data");
    assert!(response.contains("Example Domain"), "Should contain expected text");

    println!("✅ Vec callback test passed! {} bytes received", buffer_box.len());
}
