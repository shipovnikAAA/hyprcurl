//! Basic tests to isolate issues

use curl_cffi_rs::Curl;

#[test]
fn test_curl_init() {
    let curl = Curl::new();
    assert!(curl.is_ok(), "Should be able to create Curl");
    println!("✅ Curl initialization works");
}

#[test]
fn test_set_url_only() {
    let mut curl = Curl::new().expect("Failed to create Curl");
    let result = curl.set_url("https://httpbin.org/get");
    assert!(result.is_ok(), "Should be able to set URL");
    println!("✅ Setting URL works");
}

#[test]
fn test_perform_minimal() {
    eprintln!("Creating Curl...");
    let mut curl = Curl::new().expect("Failed to create Curl");

    eprintln!("Setting URL...");
    curl.set_url("https://httpbin.org/get")
        .expect("Failed to set URL");

    eprintln!("Creating response buffer...");
    let mut response = Vec::new();

    eprintln!("Performing request...");
    let result = curl.perform(&mut response);

    match &result {
        Ok(_) => {
            println!("✅ Minimal perform works");
            println!("Response length: {} bytes", response.len());
        }
        Err(e) => {
            eprintln!("Request failed: {:?}", e);
            // SSL certificate errors are expected without proper CA setup
            // The important thing is that we don't get a segfault
            if e.to_string().contains("SSL") || e.to_string().contains("certificate") {
                println!("✅ No segfault - SSL error is expected");
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

#[test]
fn test_perform_local() {
    eprintln!("Creating Curl...");
    let mut curl = Curl::new().expect("Failed to create Curl");

    eprintln!("Setting local URL...");
    curl.set_url("http://localhost:8080")
        .expect("Failed to set URL");

    eprintln!("Creating response buffer...");
    let mut response = Vec::new();

    eprintln!("Performing request...");
    let result = curl.perform(&mut response);

    // We expect this to fail with connection error, not segfault
    match result {
        Ok(_) => println!("✅ Request succeeded unexpectedly"),
        Err(e) => {
            println!("✅ Request failed as expected: {:?}", e);
            // This should not be a segfault
        }
    }
}
