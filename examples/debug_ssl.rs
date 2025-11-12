//! Debug SSL verification issue

use curl_cffi_rs::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    println!("=== Debug SSL Verification Issue ===\n");

    // Test 1: Default (should work)
    println!("1. Testing default SSL behavior...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        let mut response = Vec::new();
        match curl.perform(&mut response) {
            Ok(_) => println!("✅ Default SSL works: {} bytes", response.len()),
            Err(e) => println!("❌ Default SSL failed: {}", e),
        }
    }

    // Test 2: Explicitly enable SSL verification (should work)
    println!("\n2. Testing explicit SSL enable...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        curl.set_ssl_verify(Some(true))?;
        let mut response = Vec::new();
        match curl.perform(&mut response) {
            Ok(_) => println!("✅ SSL enabled works: {} bytes", response.len()),
            Err(e) => println!("❌ SSL enabled failed: {}", e),
        }
    }

    // Test 3: Disable SSL verification using setopt_long directly
    println!("\n3. Testing SSL verification disabled using setopt_long...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        
        println!("   Setting SslVerifyPeer to 0...");
        curl.setopt_long(curl_cffi_rs::CurlOpt::SslVerifyPeer, 0)?;
        
        println!("   Setting SslVerifyHost to 0...");
        curl.setopt_long(curl_cffi_rs::CurlOpt::SslVerifyHost, 0)?;
        
        println!("   Performing request...");
        let mut response = Vec::new();
        match curl.perform(&mut response) {
            Ok(_) => println!("✅ SSL disabled works: {} bytes", response.len()),
            Err(e) => println!("❌ SSL disabled failed: {}", e),
        }
    }

    // Test 4: Try with set_ssl_verify(false) in a safe way
    println!("\n4. Testing set_ssl_verify(false) with error handling...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        
        // Catch the panic if it happens
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            curl.set_ssl_verify(Some(false)).unwrap();
            let mut response = Vec::new();
            curl.perform(&mut response).unwrap();
            response.len()
        }));
        
        match result {
            Ok(len) => println!("✅ set_ssl_verify(false) works: {} bytes", len),
            Err(_) => println!("❌ set_ssl_verify(false) caused panic/segfault"),
        }
    }

    println!("\n=== Debug Complete ===");
    Ok(())
}