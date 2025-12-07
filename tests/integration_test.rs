//! Integration tests for curl-cffi-rs
//! These tests make real HTTP requests to httpbin.org

use curl_cffi_rs::{Curl, CurlOpt};

#[test]
fn test_simple_get_request() {
    let mut curl = Curl::new().expect("Failed to create Curl");

    curl.set_url("http://httpbin.org/get")  // Use HTTP not HTTPS for now
        .expect("Failed to set URL");

    let mut response = Vec::new();
    curl.perform(&mut response)
        .expect("Failed to perform request");

    // Check we got some data
    assert!(!response.is_empty(), "Response should not be empty");

    // Check status code
    let status = curl.response_code().expect("Failed to get status code");
    assert_eq!(status, 200, "Expected status code 200");

    // Check response contains expected JSON
    let response_str = String::from_utf8_lossy(&response);
    assert!(response_str.contains("\"url\""), "Response should contain URL field");

    println!("✅ Simple GET request successful");
    println!("Status: {}", status);
    println!("Response length: {} bytes", response.len());
}

#[test]
fn test_get_with_headers() {
    let mut curl = Curl::new().expect("Failed to create Curl");

    curl.set_url("https://httpbin.org/headers")
        .expect("Failed to set URL");

    curl.add_header("X-Custom-Header: TestValue")
        .expect("Failed to add header");

    curl.add_header("User-Agent: curl-cffi-rs/0.1.0")
        .expect("Failed to add user agent");

    let mut response = Vec::new();
    curl.perform(&mut response)
        .expect("Failed to perform request");

    let response_str = String::from_utf8_lossy(&response);

    // Check our custom header was sent
    assert!(response_str.contains("X-Custom-Header"),
            "Response should contain our custom header");
    assert!(response_str.contains("TestValue"),
            "Response should contain our header value");

    println!("✅ GET with custom headers successful");
}

#[test]
fn test_post_request() {
    let mut curl = Curl::new().expect("Failed to create Curl");

    curl.set_url("https://httpbin.org/post")
        .expect("Failed to set URL");

    // Set POST data
    let post_data = r#"{"key": "value", "test": "data"}"#;
    curl.set_post_data(post_data)
        .expect("Failed to set POST data");

    // Add content-type header
    curl.add_header("Content-Type: application/json")
        .expect("Failed to add content-type header");

    let mut response = Vec::new();
    curl.perform(&mut response)
        .expect("Failed to perform POST request");

    let response_str = String::from_utf8_lossy(&response);

    // Check the server echoed our data back
    assert!(response_str.contains("\"key\""),
            "Response should contain our posted key");
    assert!(response_str.contains("\"value\""),
            "Response should contain our posted value");

    println!("✅ POST request successful");
}

#[test]
fn test_timeout_setting() {
    let mut curl = Curl::new().expect("Failed to create Curl");

    curl.set_url("https://httpbin.org/delay/1")
        .expect("Failed to set URL");

    // Set a 5 second timeout
    curl.setopt_long(CurlOpt::Timeout, 5)
        .expect("Failed to set timeout");

    let mut response = Vec::new();
    let result = curl.perform(&mut response);

    // Should succeed with 5 second timeout for 1 second delay
    assert!(result.is_ok(), "Request should succeed with adequate timeout");

    println!("✅ Timeout setting works");
}

#[test]
fn test_response_info() {
    let mut curl = Curl::new().expect("Failed to create Curl");

    curl.set_url("https://httpbin.org/redirect/2")
        .expect("Failed to set URL");

    // Enable redirects
    curl.setopt_long(CurlOpt::FollowLocation, 1)
        .expect("Failed to enable redirects");

    let mut response = Vec::new();
    curl.perform(&mut response)
        .expect("Failed to perform request");

    // Get response info
    let status = curl.response_code().expect("Failed to get status");
    let total_time = curl.total_time().expect("Failed to get total time");
    let effective_url = curl.effective_url().expect("Failed to get effective URL");

    assert_eq!(status, 200);
    assert!(total_time > 0.0, "Total time should be positive");
    assert!(effective_url.contains("httpbin.org"), "Effective URL should contain httpbin.org");

    println!("✅ Response info retrieval successful");
    println!("  Status: {}", status);
    println!("  Time: {:.3}s", total_time);
    println!("  Final URL: {}", effective_url);
}

#[test]
fn test_handle_reuse() {
    let mut curl = Curl::new().expect("Failed to create Curl");

    // First request
    curl.set_url("https://httpbin.org/get")
        .expect("Failed to set URL");

    let mut response1 = Vec::new();
    curl.perform(&mut response1)
        .expect("Failed to perform first request");

    assert!(!response1.is_empty());

    // Reset and reuse
    curl.reset();

    // Second request
    curl.set_url("https://httpbin.org/uuid")
        .expect("Failed to set URL for second request");

    let mut response2 = Vec::new();
    curl.perform(&mut response2)
        .expect("Failed to perform second request");

    assert!(!response2.is_empty());
    assert_ne!(response1, response2, "Two requests should have different responses");

    println!("✅ Handle reuse works correctly");
}

#[test]
fn test_user_agent() {
    let mut curl = Curl::new().expect("Failed to create Curl");

    curl.set_url("https://httpbin.org/user-agent")
        .expect("Failed to set URL");

    let custom_ua = "curl-cffi-rs-test/1.0";
    curl.setopt_str(CurlOpt::UserAgent, custom_ua)
        .expect("Failed to set user agent");

    let mut response = Vec::new();
    curl.perform(&mut response)
        .expect("Failed to perform request");

    let response_str = String::from_utf8_lossy(&response);

    assert!(response_str.contains(custom_ua),
            "Response should contain our custom user agent");

    println!("✅ User agent setting works");
}

#[test]
#[ignore] // This test requires a slow connection or will timeout
fn test_timeout_failure() {
    let mut curl = Curl::new().expect("Failed to create Curl");

    curl.set_url("https://httpbin.org/delay/10")
        .expect("Failed to set URL");

    // Set a very short timeout (1 second for 10 second delay)
    curl.setopt_long(CurlOpt::Timeout, 1)
        .expect("Failed to set timeout");

    let mut response = Vec::new();
    let result = curl.perform(&mut response);

    // Should fail with timeout
    assert!(result.is_err(), "Request should timeout");

    println!("✅ Timeout error handling works");
}
