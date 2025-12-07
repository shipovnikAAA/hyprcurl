[![Crates.io](https://img.shields.io/crates/v/hyprcurl.svg)](https://crates.io/crates/hyprcurl)
[![Documentation](https://docs.rs/hyprcurl/badge.svg)](https://docs.rs/hyprcurl)
[![License](https://img.shields.io/crates/l/hyprcurl.svg)](https://github.com/Aditya-PS-05/hyprcurl#license)

# hyprcurl

A high-performance Rust HTTP client with browser impersonation and TLS fingerprinting support.

> Rust implementation inspired by Python's [curl_cffi](https://github.com/yifeikong/curl_cffi)

## Installation

### Rust

```bash
cargo add hyprcurl
```

### Python

```bash
# Build from source
cargo build --release --features python
pip install maturin
maturin develop --release
```

## Quick Start

### Rust - Simple Requests

```rust
use hyprcurl::{get, post};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple GET
    let response = get("https://httpbin.org/get")?;
    println!("{}", String::from_utf8_lossy(&response));

    // Simple POST
    let data = r#"{"key": "value"}"#;
    let response = post("https://httpbin.org/post", data)?;

    Ok(())
}
```

### Rust - Request Builder API

```rust
use hyprcurl::{Request, Browser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET with browser impersonation
    let response = Request::get("https://tls.browserleaks.com/json")
        .impersonate(Browser::ChromeLatest)
        .send()?;

    // POST with browser impersonation and proxy
    let response = Request::post("https://httpbin.org/post", r#"{"test": "data"}"#)
        .impersonate(Browser::FirefoxLatest)
        .proxies("socks5://localhost:1080")
        .send()?;

    Ok(())
}
```

### Rust - Low-level API

```rust
use hyprcurl::{Curl, Browser, CurlOpt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut curl = Curl::new()?;

    // Configure request
    curl.set_url("https://httpbin.org/get")?;
    curl.set_browser_impersonation(Browser::ChromeLatest)?;
    curl.add_header("X-Custom-Header: value")?;
    curl.setopt_long(CurlOpt::Timeout, 30)?;

    // Perform request
    let mut buffer = Vec::new();
    curl.perform(&mut buffer)?;

    // Get response metadata
    let status = curl.response_code()?;
    let time = curl.total_time()?;

    println!("Status: {}, Time: {:.2}s", status, time);
    Ok(())
}
```

### Python

```python
import hyprcurl

# Simple GET
response = hyprcurl.get("https://httpbin.org/get")

# GET with browser impersonation
response = hyprcurl.get(
    "https://tls.browserleaks.com/json",
    impersonate="chrome"
)

# POST with impersonation and proxy
response = hyprcurl.post(
    "https://httpbin.org/post",
    data='{"key": "value"}',
    impersonate="firefox121",
    proxies="socks5://localhost:1080"
)
```

## Browser Impersonation

Supported browsers:

```rust
Browser::ChromeLatest       // Chrome 131
Browser::Chrome { version: 110 }
Browser::FirefoxLatest      // Firefox 121
Browser::Firefox { version: 115 }
Browser::SafariLatest       // Safari 18.0
Browser::Safari { version: "17.5".into() }
Browser::EdgeLatest         // Edge 131
Browser::Edge { version: 120 }
```

Each browser impersonation sets:
- User-Agent header
- HTTP/2 settings
- TLS cipher suites
- TLS curves (including X25519Kyber768 for Chrome)
- SSL/TLS signature algorithms
- ALPN protocols

## Dependencies

- **libcurl** - System libcurl installation required
- **Rust 1.70+** - For building from source
- **Python 3.8+** - For Python bindings (optional)

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run integration tests (makes real HTTP requests)
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

## Benchmarks

Compare performance with Python curl_cffi:

### 1. Start the test server

```bash
cd benchmarks
uv init
uv add curl_cffi pandas matplotlib
uv run server.py
```

### 2. Build Rust library

```bash
cargo build --release --features python
cp target/release/libhyprcurl.so target/release/hyprcurl.so
```

### 3. Run benchmarks

```bash
cd benchmarks
uv run python_vs_rust_bench.py
```

Results:
- CSV file: `python_vs_rust_bench.csv`
- Chart: `python_vs_rust_bench.png`
- Console output with speedup analysis

## Documentation

### Book-style Documentation

Build and view the comprehensive guide:

```bash
# Install mdBook
cargo install mdbook

# Build and serve the book
mdbook serve book --open
# Opens at http://localhost:3000
```

### API Documentation

Generate Rust API docs:

```bash
# Standard docs
cargo doc --no-deps --open

# With all features
cargo doc --all-features --open
```

## Examples

See the [`examples/`](examples/) directory for complete working examples:

- [`simple_get.rs`](examples/simple_get.rs) - Basic GET requests
- [`browser_impersonation.rs`](examples/browser_impersonation.rs) - All browser types
- [`proxy_usage.rs`](examples/proxy_usage.rs) - HTTP and SOCKS5 proxies
- [`post_request.rs`](examples/post_request.rs) - POST with various options
- [`comprehensive.rs`](examples/comprehensive.rs) - Full feature showcase

Run examples:
```bash
cargo run --example simple_get
cargo run --example browser_impersonation
```

## Acknowledgments

Inspired by [curl_cffi](https://github.com/yifeikong/curl_cffi) - the excellent Python library for browser impersonation.
