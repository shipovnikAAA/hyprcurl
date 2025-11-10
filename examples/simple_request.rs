//! Simple HTTP GET request example

use curl_cffi_rs::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    println!("=== Simple HTTP GET Request ===\n");

    // Create a new Curl instance
    let mut curl = Curl::new()?;

    // Set the URL
    curl.set_url("https://httpbin.org/get")?;

    // Add some headers
    curl.add_header("Accept: application/json")?;
    curl.add_header("User-Agent: curl-cffi-rs/0.1.0")?;

    // Set timeout
    curl.setopt_long(curl_cffi_rs::types::CurlOpt::Timeout, 30)?;
    
    // SSL verification is enabled by default (secure)
    // For testing with invalid certificates, you could use:
    // curl.set_ssl_verify(Some(false))?;

    // Perform the request
    let mut response = Vec::new();
    println!("Fetching URL...");
    curl.perform(&mut response)?;

    // Get response info
    let status_code = curl.response_code()?;
    let total_time = curl.total_time()?;
    let effective_url = curl.effective_url()?;

    println!("Status Code: {}", status_code);
    println!("Total Time: {:.3}s", total_time);
    println!("Effective URL: {}", effective_url);
    println!("\nResponse Body:");
    println!("{}", String::from_utf8_lossy(&response));

    Ok(())
}
