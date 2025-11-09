//! Async HTTP requests example

use curl_cffi_rs::{AsyncCurl, Curl, CurlError};

#[tokio::main]
async fn main() -> Result<(), CurlError> {
    println!("=== Async HTTP Requests ===\n");

    let async_curl = AsyncCurl::new()?;

    // Create multiple requests
    let urls = vec![
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/1",
    ];

    let mut curls = Vec::new();
    for url in &urls {
        let mut curl = Curl::new()?;
        curl.set_url(url)?;
        curls.push(curl);
    }

    println!("Fetching {} URLs concurrently...", urls.len());
    let start = std::time::Instant::now();

    // Perform all requests concurrently
    let results = async_curl.perform_many(curls).await;

    let elapsed = start.elapsed();
    println!("Total time: {:.3}s", elapsed.as_secs_f64());
    println!("(Sequential would take ~3s, concurrent takes ~1s)");

    // Check results
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(data) => println!("Request {}: {} bytes", i + 1, data.len()),
            Err(e) => println!("Request {}: Error - {}", i + 1, e),
        }
    }

    Ok(())
}
