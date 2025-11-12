//! Simple SSL test to isolate the issue

use curl_cffi_rs::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    println!("=== Simple SSL Test ===\n");

    // Test 1: Default secure behavior
    println!("1. Testing default secure SSL...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ Default SSL: Status {}", curl.response_code()?);
    }

    // Test 2: Explicitly enable SSL verification
    println!("\n2. Testing explicit SSL enable...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        curl.set_ssl_verify(Some(true))?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ SSL enabled: Status {}", curl.response_code()?);
    }

    // Test 3: Disable SSL verification (this might cause the segfault)
    println!("\n3. Testing SSL verification disabled...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        curl.set_ssl_verify(Some(false))?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ SSL disabled: Status {}", curl.response_code()?);
    }

    println!("\n✅ All SSL tests completed successfully!");
    Ok(())
}