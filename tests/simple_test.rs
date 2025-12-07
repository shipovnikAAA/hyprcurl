//! Simplest possible test - no custom callback

use curl_cffi_rs::Curl;

#[test]
fn test_curl_with_default_callback() {
    eprintln!("Creating Curl...");
    let mut curl = Curl::new().expect("Failed to create Curl");

    eprintln!("Setting URL...");
    curl.set_url("http://example.com")  // Use HTTP not HTTPS
        .expect("Failed to set URL");

    eprintln!("Performing request WITHOUT custom callback...");
    unsafe {
        // Don't set any callback - let curl use default
        let code = curl_sys::curl_easy_perform(curl.raw_handle());
        if code != 0 {
            eprintln!("Perform failed with code: {}", code);
        } else {
            eprintln!("✅ SUCCESS! Request completed");
        }
    }

    eprintln!("Getting response code...");
    let status = curl.response_code();
    eprintln!("Status result: {:?}", status);

    println!("✅ Test completed without crash");
}
