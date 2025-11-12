//! Test minimal SSL verification changes

use curl_cffi_rs::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    println!("=== Test Minimal SSL Changes ===\n");

    // Test 1: Only disable peer verification
    println!("1. Testing SslVerifyPeer=0 only...");
    {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut curl = Curl::new().unwrap();
            curl.set_url("https://httpbin.org/get").unwrap();
            
            // Only disable peer verification
            curl.setopt_long(curl_cffi_rs::CurlOpt::SslVerifyPeer, 0).unwrap();
            
            let mut response = Vec::new();
            curl.perform(&mut response).unwrap();
            response.len()
        }));
        
        match result {
            Ok(len) => println!("   ✅ Works: {} bytes", len),
            Err(_) => println!("   ❌ Crashes"),
        }
    }

    // Test 2: Only disable host verification  
    println!("\n2. Testing SslVerifyHost=0 only...");
    {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut curl = Curl::new().unwrap();
            curl.set_url("https://httpbin.org/get").unwrap();
            
            // Only disable host verification
            curl.setopt_long(curl_cffi_rs::CurlOpt::SslVerifyHost, 0).unwrap();
            
            let mut response = Vec::new();
            curl.perform(&mut response).unwrap();
            response.len()
        }));
        
        match result {
            Ok(len) => println!("   ✅ Works: {} bytes", len),
            Err(_) => println!("   ❌ Crashes"),
        }
    }

    // Test 3: Try with CURLOPT_SSL_VERIFYPEER as string instead of long
    println!("\n3. Testing with CURLOPT_SSL_VERIFYPEER as string...");
    {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut curl = Curl::new().unwrap();
            curl.set_url("https://httpbin.org/get").unwrap();
            
            // Try setting as string (shouldn't work but let's see)
            curl.setopt_str(curl_cffi_rs::CurlOpt::SslVerifyPeer, "0").unwrap();
            
            let mut response = Vec::new();
            curl.perform(&mut response).unwrap();
            response.len()
        }));
        
        match result {
            Ok(len) => println!("   ✅ Works: {} bytes", len),
            Err(_) => println!("   ❌ Crashes"),
        }
    }

    Ok(())
}