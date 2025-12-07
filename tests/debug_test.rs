use curl_cffi_rs::{Curl, CurlOpt};

#[test]
fn test_debug_segfault() {
    println!("Testing basic curl creation...");
    
    // Test 1: Just create curl
    {
        let curl = Curl::new();
        assert!(curl.is_ok(), "Should be able to create Curl");
        println!("✅ Curl creation works");
    }
    
    // Test 2: Create and set URL
    {
        let mut curl = Curl::new().expect("Failed to create Curl");
        let result = curl.set_url("https://example.com");
        assert!(result.is_ok(), "Should be able to set URL");
        println!("✅ Setting URL works");
    }
    
    // Test 3: Try to get version info
    {
        let version = curl_cffi_rs::version();
        println!("✅ Curl version: {}", version);
    }
    
    
    // Test 4: Test perform with a file:// URL (no network)
    {
        let mut curl = Curl::new().expect("Failed to create Curl");
        
        // Use file:// URL to test without network
        let result = curl.set_url("file:///etc/hostname");
        assert!(result.is_ok(), "Should be able to set file URL");
        
        let mut response = Vec::new();
        let perform_result = curl.perform(&mut response);
        
        match perform_result {
            Ok(_) => {
                println!("✅ File request works, response length: {}", response.len());
                if !response.is_empty() {
                    println!("Response content: {}", String::from_utf8_lossy(&response));
                }
            }
            Err(e) => {
                println!("File request failed: {:?}", e);
                // This is okay, we just want to avoid segfaults
            }
        }
    }
    
    // Test 5: Test perform with HTTP URL
    {
        let mut curl = Curl::new().expect("Failed to create Curl");
        
        let result = curl.set_url("http://example.com");
        assert!(result.is_ok(), "Should be able to set HTTP URL");
        
        let mut response = Vec::new();
        let perform_result = curl.perform(&mut response);
        
        match perform_result {
            Ok(_) => {
                println!("✅ HTTP request works, response length: {}", response.len());
            }
            Err(e) => {
                println!("HTTP request failed: {:?}", e);
                // This should not be a segfault
            }
        }
    }
    
    // Test 6: Test perform with HTTPS URL
    {
        let mut curl = Curl::new().expect("Failed to create Curl");
        
        let result = curl.set_url("https://example.com");
        assert!(result.is_ok(), "Should be able to set HTTPS URL");
        
        // Disable SSL verification for testing
        let _ = curl.setopt_long(CurlOpt::SslVerifyPeer, 0);
        let _ = curl.setopt_long(CurlOpt::SslVerifyHost, 0);
        
        let mut response = Vec::new();
        let perform_result = curl.perform(&mut response);
        
        match perform_result {
            Ok(_) => {
                println!("✅ HTTPS request works, response length: {}", response.len());
            }
            Err(e) => {
                println!("HTTPS request failed: {:?}", e);
                // This should not be a segfault
            }
        }
    }
    
    println!("All basic tests passed!");
}