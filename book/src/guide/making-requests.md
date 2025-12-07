# Making Requests

This chapter covers the details of making HTTP requests with curl-cffi-rs.

## Request Methods

curl-cffi-rs supports all standard HTTP methods.

### GET

```rust
let mut curl = Curl::new()?;
curl.set_url("https://api.example.com/data")?;

let mut response = Vec::new();
curl.perform(&mut response)?;
```

### POST

```rust
let mut curl = Curl::new()?;
curl.set_url("https://api.example.com/data")?;

// JSON data
let data = r#"{"key": "value"}"#;
curl.set_post_fields(data)?;
curl.add_header("Content-Type: application/json")?;

let mut response = Vec::new();
curl.perform(&mut response)?;
```

### Custom Methods

```rust
// PUT
curl.set_custom_request("PUT")?;

// PATCH
curl.set_custom_request("PATCH")?;

// DELETE
curl.set_custom_request("DELETE")?;
```

## Request Headers

### Adding Headers

```rust
curl.add_header("User-Agent: MyApp/1.0")?;
curl.add_header("Accept: application/json")?;
curl.add_header("X-Custom-Header: value")?;
```

### Common Header Patterns

```rust
// JSON API
curl.add_header("Content-Type: application/json")?;
curl.add_header("Accept: application/json")?;

// Form data
curl.add_header("Content-Type: application/x-www-form-urlencoded")?;

// Authentication
curl.add_header("Authorization: Bearer YOUR_TOKEN")?;

// CORS
curl.add_header("Origin: https://example.com")?;
```

## Request Body

### JSON Data

```rust
use serde_json::json;

let data = json!({
    "name": "curl-cffi-rs",
    "version": "0.1.0"
});

curl.set_post_fields(&data.to_string())?;
curl.add_header("Content-Type: application/json")?;
```

### Form Data

```rust
let form_data = "username=john&password=secret";
curl.set_post_fields(form_data)?;
curl.add_header("Content-Type: application/x-www-form-urlencoded")?;
```

### Binary Data

```rust
use std::fs;

let binary_data = fs::read("file.bin")?;
curl.set_post_fields_binary(&binary_data)?;
```

## Query Parameters

### Manual Construction

```rust
let url = "https://api.example.com/search?q=rust&limit=10";
curl.set_url(url)?;
```

### Using URL Builder

```rust
use url::Url;

let mut url = Url::parse("https://api.example.com/search")?;
url.query_pairs_mut()
    .append_pair("q", "rust")
    .append_pair("limit", "10");

curl.set_url(url.as_str())?;
```

## Response Handling

### Reading Response Body

```rust
let mut response = Vec::new();
curl.perform(&mut response)?;

let body = String::from_utf8_lossy(&response);
println!("{}", body);
```

### Parsing JSON

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    status: String,
    data: Vec<String>,
}

let mut response = Vec::new();
curl.perform(&mut response)?;

let parsed: ApiResponse = serde_json::from_slice(&response)?;
```

### Streaming Response

For large responses, consider processing data as it arrives:

```rust
// Future feature - streaming support
// Currently, data is buffered in memory
```

## Connection Options

### Timeouts

```rust
// Connection timeout (seconds)
curl.set_connect_timeout(30)?;

// Total request timeout (seconds)
curl.set_timeout(60)?;
```

### Redirects

```rust
// Follow redirects (default: true)
curl.set_follow_redirects(true)?;

// Maximum redirects
curl.set_max_redirects(10)?;
```

### Keep-Alive

```rust
// Reuse the same Curl instance for connection pooling
let mut curl = Curl::new()?;

for i in 0..10 {
    curl.set_url(&format!("https://api.example.com/item/{}", i))?;
    let mut response = Vec::new();
    curl.perform(&mut response)?;
    // Connection is reused
}
```

## Error Handling

### Checking Status Codes

```rust
curl.perform(&mut response)?;

let status = curl.get_response_code()?;
if status >= 400 {
    eprintln!("HTTP error: {}", status);
    return Err(CurlError::Other(format!("HTTP {}", status)));
}
```

### Catching Network Errors

```rust
match curl.perform(&mut response) {
    Ok(_) => println!("Success!"),
    Err(CurlError::CurlCode { code, message }) => {
        eprintln!("Network error {}: {}", code, message);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Next Steps

- [Browser Impersonation](./browser-impersonation.md) - Mimic real browsers
- [SSL/TLS Configuration](./ssl-tls.md) - Secure connections
- [Headers and Cookies](./headers-cookies.md) - Advanced header management
