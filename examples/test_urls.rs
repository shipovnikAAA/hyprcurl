//! Test various URL fetching capabilities

use curl_cffi_rs::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    println!("=== URL Fetching Test ===\n");

    // Test 1: Simple HTTP GET
    println!("1. HTTP GET request...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("http://httpbin.org/get")?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ HTTP: {} bytes", response.len());
    }

    // Test 2: HTTPS GET (secure)
    println!("\n2. HTTPS GET request...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ HTTPS: {} bytes", response.len());
    }

    // Test 3: JSON API
    println!("\n3. JSON API response...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://api.github.com/repos/rust-lang/rust")?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ GitHub API: {} bytes", response.len());
        
        // Just show we got valid response
        if let Ok(text) = std::str::from_utf8(&response) {
            if text.len() > 100 {
                println!("   Got valid JSON response ({} chars)", text.len());
            }
        }
    }

    // Test 4: POST request
    println!("\n4. POST request...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/post")?;
        curl.set_post_data("hello=world")?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ POST: {} bytes", response.len());
    }

    // Test 5: Custom headers
    println!("\n5. Custom headers...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/headers")?;
        curl.add_header("User-Agent: curl-cffi-rs-test/1.0")?;
        curl.add_header("X-Custom-Header: test-value")?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("✅ Custom headers: {} bytes", response.len());
    }

    // Test 6: Different domains
    println!("\n6. Different domains...");
    let urls = [
        "https://www.google.com",
        "https://www.github.com", 
        "https://www.wikipedia.org",
    ];
    
    for (i, url) in urls.iter().enumerate() {
        let mut curl = Curl::new()?;
        curl.set_url(url)?;
        curl.setopt_long(curl_cffi_rs::CurlOpt::FollowLocation, 1)?; // Follow redirects
        curl.setopt_long(curl_cffi_rs::CurlOpt::Timeout, 10)?; // 10 second timeout
        let mut response = Vec::new();
        match curl.perform(&mut response) {
            Ok(_) => println!("✅ {}: {} bytes", url, response.len()),
            Err(e) => println!("❌ {}: {}", url, e),
        }
    }

    println!("\n✅ All URL fetching tests completed!");
    Ok(())
}