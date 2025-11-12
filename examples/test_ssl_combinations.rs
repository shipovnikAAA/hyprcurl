//! Test different SSL verification combinations

use curl_cffi_rs::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    println!("=== Test SSL Verification Combinations ===\n");

    let test_cases = vec![
        ("SslVerifyPeer=0, SslVerifyHost=2", 0, 2),
        ("SslVerifyPeer=1, SslVerifyHost=0", 1, 0),
        ("SslVerifyPeer=0, SslVerifyHost=1", 0, 1),
        ("SslVerifyPeer=0, SslVerifyHost=0", 0, 0), // This should crash
    ];

    for (i, (name, peer, host)) in test_cases.iter().enumerate() {
        println!("{}. Testing {}...", i + 1, name);
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut curl = Curl::new().unwrap();
            curl.set_url("https://httpbin.org/get").unwrap();
            
            curl.setopt_long(curl_cffi_rs::CurlOpt::SslVerifyPeer, *peer).unwrap();
            curl.setopt_long(curl_cffi_rs::CurlOpt::SslVerifyHost, *host).unwrap();
            
            let mut response = Vec::new();
            curl.perform(&mut response).unwrap();
            response.len()
        }));
        
        match result {
            Ok(len) => println!("   ✅ Works: {} bytes", len),
            Err(_) => println!("   ❌ Crashes with segfault"),
        }
        
        println!();
    }

    // Test the fix: Only disable peer verification, keep host verification
    println!("5. Testing fix (disable peer only, keep host verification)...");
    {
        let mut curl = Curl::new()?;
        curl.set_url("https://httpbin.org/get")?;
        
        // Only disable peer verification, keep host verification at 1
        curl.setopt_long(curl_cffi_rs::CurlOpt::SslVerifyPeer, 0)?;
        curl.setopt_long(curl_cffi_rs::CurlOpt::SslVerifyHost, 1)?;
        
        let mut response = Vec::new();
        match curl.perform(&mut response) {
            Ok(_) => println!("   ✅ Fix works: {} bytes", response.len()),
            Err(e) => println!("   ❌ Fix failed: {}", e),
        }
    }

    Ok(())
}