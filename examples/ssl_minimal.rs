//! Minimal SSL test exactly like Python curl-cffi

use curl_cffi_rs::{Curl, CurlError};
use curl_cffi_rs::types::CurlOpt;

fn main() -> Result<(), CurlError> {
    println!("=== Minimal SSL Test (Python-style) ===\n");

    // Test 1: Default behavior (should work)
    println!("1. Testing default SSL verification...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ Default SSL: Status {}", curl.response_code()?);
    }

    // Test 2: Disable SSL verification exactly like Python
    println!("\n2. Disabling SSL verification (Python style)...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        
        // Exactly like Python curl-cffi does it
        curl.setopt_long(CurlOpt::SslVerifyPeer, 0)?;
        curl.setopt_long(CurlOpt::SslVerifyHost, 0)?;
        
        println!("SSL verification disabled, attempting request...");
        let mut response = Vec::new();
        match curl.perform(&mut response) {
            Ok(_) => println!("✅ SSL disabled: Status {}", curl.response_code()?),
            Err(e) => println!("❌ SSL disabled failed: {}", e),
        }
    }

    println!("\n✅ Test completed!");
    Ok(())
}