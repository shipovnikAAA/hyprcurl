# Introduction

Welcome to the **curl-cffi-rs** guide! This book will teach you everything you need to know about using curl-cffi-rs, a high-performance Rust implementation of curl_cffi with browser fingerprinting support.

## What is curl-cffi-rs?

curl-cffi-rs is a Rust library that provides:

- **Zero-cost abstractions** - Efficient wrapper around libcurl
- **Type-safe API** - Leverage Rust's type system for safer HTTP requests
- **Browser Impersonation** - Mimic real browser TLS fingerprints to avoid bot detection
- **Async/await support** - First-class async support via tokio
- **WebSocket support** - Built-in WebSocket client
- **Python bindings** - Use from Python via PyO3

## Why curl-cffi-rs?

Modern websites often use sophisticated bot detection mechanisms that analyze TLS fingerprints and HTTP/2 characteristics. curl-cffi-rs uses libcurl-impersonate to mimic real browsers at the TLS level, making it ideal for:

- Web scraping
- API testing
- Automation tasks
- Research and security testing

## Who is this book for?

This book is for anyone who wants to:

- Make HTTP requests in Rust with browser-like characteristics
- Bypass basic bot detection mechanisms
- Build high-performance web scrapers
- Use WebSockets with browser impersonation

## How to use this book

This book is organized into several sections:

- **Getting Started** - Installation and basic usage
- **Guide** - Common tasks and features
- **Advanced** - Async support, WebSockets, and Python bindings
- **API Reference** - Detailed API documentation

You can read the book linearly or jump to specific topics as needed.

## Contributing

curl-cffi-rs is open source! Contributions are welcome. Visit our [GitHub repository](https://github.com/Aditya-PS-05/hyprcurl) to get involved.
