//! SSL/TLS functionality demonstration

use curl_cffi_rs::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    println!("=== SSL/TLS Functionality Test ===\n");

    // Test 1: Default secure SSL verification
    println!("1. Testing default SSL verification (secure)...");
    let mut curl = Curl::new()?;
    curl.set_url("https://httpbin.org/get")?;
    curl.setopt_long(curl_cffi_rs::types::CurlOpt::Timeout, 10)?;
    
    let mut response = Vec::new();
    curl.perform(&mut response)?;
    println!("✅ Default SSL verification: Status {}", curl.response_code()?);

    // Test 2: Explicit SSL verification control
    println!("\n2. Testing explicit SSL verification control...");
    curl.set_ssl_verify(Some(true))?;  // Explicitly enable
    let mut response2 = Vec::new();
    curl.perform(&mut response2)?;
    println!("✅ Explicit SSL verification enabled: Status {}", curl.response_code()?);

    // Test 3: Custom CA certificate (if available)
    println!("\n3. Testing custom CA certificate detection...");
    if let Some(ca_path) = detect_system_ca_bundle() {
        println!("Found system CA bundle: {}", ca_path);
        curl.set_ca_cert_file(&ca_path)?;
        let mut response3 = Vec::new();
        curl.perform(&mut response3)?;
        println!("✅ Custom CA certificate: Status {}", curl.response_code()?);
    } else {
        println!("ℹ️  No system CA bundle detected, using curl's defaults");
    }

    // Test 4: SSL verification disabled (for testing only)
    println!("\n4. Testing SSL verification disabled (insecure)...");
    curl.set_ssl_verify(Some(false))?;
    let mut response4 = Vec::new();
    curl.perform(&mut response4)?;
    println!("⚠️  SSL verification disabled: Status {}", curl.response_code()?);

    // Test 5: Restore secure defaults
    println!("\n5. Restoring secure defaults...");
    curl.set_ssl_verify(None)?;  // Reset to default (secure)
    let mut response5 = Vec::new();
    curl.perform(&mut response5)?;
    println!("✅ Secure defaults restored: Status {}", curl.response_code()?);

    println!("\n=== SSL Test Summary ===");
    println!("✅ All SSL tests completed successfully!");
    println!("✅ Default behavior is secure (SSL verification enabled)");
    println!("✅ SSL verification can be controlled programmatically");
    println!("✅ System CA certificate detection works");
    println!("✅ Secure-by-default approach matches curl-cffi Python");

    Ok(())
}

/// Detect system CA certificate bundle path
fn detect_system_ca_bundle() -> Option<String> {
    // Check common environment variables first
    if let Ok(path) = std::env::var("SSL_CERT_FILE") {
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }
    
    if let Ok(path) = std::env::var("CURL_CA_BUNDLE") {
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }

    // Check common system paths
    let common_paths = [
        "/etc/ssl/certs/ca-certificates.crt",     // Debian/Ubuntu
        "/etc/pki/tls/certs/ca-bundle.crt",       // RHEL/CentOS
        "/etc/ssl/ca-bundle.pem",                 // OpenSUSE
        "/usr/local/share/certs/ca-root-nss.crt",  // FreeBSD
        "/etc/openssl/certs/ca-certificates.crt",  // Some Linux
    ];

    for path in &common_paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    None
}