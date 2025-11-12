//! Debug SSL issue by testing individual curl options

use curl_cffi_rs::{Curl, CurlError};
use curl_cffi_rs::types::CurlOpt;

fn main() -> Result<(), CurlError> {
    println!("=== SSL Debug Test ===\n");

    // Test setting SSL options directly
    println!("1. Testing direct SSL option setting...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        
        // Set SSL verification options directly
        curl.setopt_long(CurlOpt::SslVerifyPeer, 0)?;
        curl.setopt_long(CurlOpt::SslVerifyHost, 0)?;
        
        println!("✅ SSL options set, attempting request...");
        let mut response = Vec::new();
        match curl.perform(&mut response) {
            Ok(_) => println!("✅ Request completed: Status {}", curl.response_code()?),
            Err(e) => println!("❌ Request failed: {}", e),
        }
    }

    println!("\n2. Testing with SSL verification enabled...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        
        // Enable SSL verification
        curl.setopt_long(CurlOpt::SslVerifyPeer, 1)?;
        curl.setopt_long(CurlOpt::SslVerifyHost, 2)?;
        
        println!("✅ SSL verification enabled, attempting request...");
        let mut response = Vec::new();
        match curl.perform(&mut response) {
            Ok(_) => println!("✅ Request completed: Status {}", curl.response_code()?),
            Err(e) => println!("❌ Request failed: {}", e),
        }
    }

    println!("\n✅ Debug test completed!");
    Ok(())
}