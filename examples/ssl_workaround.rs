//! SSL verification workaround example

use curl_cffi_rs::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    println!("=== SSL Verification Workarounds ===\n");

    // Method 1: Use default SSL verification (recommended)
    println!("1. Default SSL verification (recommended)...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("   ✅ Works: {} bytes", response.len());
    }

    // Method 2: For testing with self-signed certs, create a dummy CA file
    println!("\n2. Using custom CA file for testing...");
    {
        // Create a temporary CA file (this would normally contain your test cert)
        use std::fs;
        use std::io::Write;
        
        let temp_ca_path = "/tmp/test_ca.pem";
        if let Ok(_) = fs::write(temp_ca_path, "# Dummy CA file for testing\n") {
            let mut curl = Curl::new()?;
            curl.set_url("https://httpbin.org/get")?;
            curl.set_ca_cert_file(temp_ca_path)?;
            
            let mut response = Vec::new();
            match curl.perform(&mut response) {
                Ok(_) => println!("   ✅ Custom CA works: {} bytes", response.len()),
                Err(e) => println!("   ⚠️  Custom CA failed (expected): {}", e),
            }
            
            // Clean up
            let _ = fs::remove_file(temp_ca_path);
        }
    }

    // Method 3: Try to disable SSL verification (will show error)
    println!("\n3. Attempting to disable SSL verification...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        
        match curl.set_ssl_verify(Some(false)) {
            Ok(_) => {
                let mut response = Vec::new();
                match curl.perform(&mut response) {
                    Ok(_) => println!("   ✅ SSL disabled works: {} bytes", response.len()),
                    Err(e) => println!("   ❌ SSL disabled failed: {}", e),
                }
            }
            Err(e) => {
                println!("   ⚠️  Cannot disable SSL: {}", e);
                println!("   💡 This is a safety feature to prevent segfaults");
            }
        }
    }

    // Method 4: HTTP instead of HTTPS (no SSL needed)
    println!("\n4. Using HTTP instead of HTTPS (no SSL verification needed)...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("http://httpbin.org/get")?; // HTTP instead of HTTPS
        let mut response = Vec::new();
        curl.perform(&mut response)?;
        println!("   ✅ HTTP works: {} bytes", response.len());
    }

    println!("\n=== Summary ===");
    println!("✅ SSL verification works properly (secure by default)");
    println!("⚠️  Disabling SSL verification is blocked (prevents segfault)");
    println!("💡 Use HTTP for testing, or custom CA files for specific certs");
    
    Ok(())
}