//! Browser impersonation example
//! 
//! This example demonstrates how to use curl-cffi-rs to impersonate different browsers
//! and test the browser fingerprinting capabilities.

use curl_cffi_rs::curl::Curl;
use curl_cffi_rs::types::Browser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Browser Impersonation Demo ===\n");

    // Test URLs
    let urls = vec![
        "https://httpbin.org/user-agent",
        "https://httpbin.org/headers",
    ];

    let browsers = vec![
        Browser::Chrome { version: 120 },
        Browser::Firefox { version: 121 },
        Browser::Edge { version: 120 },
        Browser::Safari { version: "17.0".to_string() },
    ];

    for (i, browser) in browsers.iter().enumerate() {
        println!("{}. Testing {:?}\n", i + 1, browser);
        
        for url in &urls {
            println!("Testing URL: {}", url);
            
            let mut curl = Curl::new()?;
            
            // Set browser impersonation
            curl.set_browser_impersonation(browser.clone())?;
            curl.set_url(url)?;
            
            // Create buffer for response
            let mut response_buffer = Vec::new();
            
            // Perform the request
            match curl.perform(&mut response_buffer) {
                Ok(_) => {
                    let status_code = curl.response_code()?;
                    println!("Status: {}", status_code);
                    println!("Response preview: {}...\n", 
                            String::from_utf8_lossy(&response_buffer).chars().take(200).collect::<String>());
                }
                Err(e) => {
                    println!("Error: {}\n", e);
                }
            }
        }
        
        println!("{}", "=".repeat(50));
    }

    Ok(())
}