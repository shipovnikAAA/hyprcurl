//! Benchmarks for curl-cffi-rs

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use curl_cffi_rs::{Curl, CurlError};

fn bench_simple_request(c: &mut Criterion) {
    c.bench_function("simple_get_request", |b| {
        b.iter(|| {
            let mut curl = Curl::new().unwrap();
            curl.set_url(black_box("http://localhost:8000/1k")).unwrap();
            let mut buffer = Vec::new();
            curl.perform(&mut buffer).unwrap();
            black_box(buffer);
        });
    });
}

fn bench_request_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("request_sizes");

    for size in ["1k", "20k", "200k"].iter() {
        let url = format!("http://localhost:8000/{}", size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &url, |b, url| {
            b.iter(|| {
                let mut curl = Curl::new().unwrap();
                curl.set_url(black_box(url)).unwrap();
                let mut buffer = Vec::new();
                curl.perform(&mut buffer).unwrap();
                black_box(buffer);
            });
        });
    }
    group.finish();
}

fn bench_handle_reuse(c: &mut Criterion) {
    c.bench_function("handle_reuse", |b| {
        let mut curl = Curl::new().unwrap();

        b.iter(|| {
            curl.reset();
            curl.set_url(black_box("http://localhost:8000/1k")).unwrap();
            let mut buffer = Vec::new();
            curl.perform(&mut buffer).unwrap();
            black_box(buffer);
        });
    });
}

fn bench_type_conversions(c: &mut Criterion) {
    c.bench_function("setopt_operations", |b| {
        b.iter(|| {
            let mut curl = Curl::new().unwrap();
            curl.set_url(black_box("https://example.com")).unwrap();
            curl.add_header(black_box("User-Agent: test")).unwrap();
            curl.add_header(black_box("Accept: application/json")).unwrap();
            curl.setopt_long(curl_cffi_rs::types::CurlOpt::Timeout, black_box(30)).unwrap();
            curl.setopt_long(curl_cffi_rs::types::CurlOpt::FollowLocation, black_box(1)).unwrap();
            black_box(curl);
        });
    });
}

criterion_group!(
    benches,
    bench_simple_request,
    bench_request_sizes,
    bench_handle_reuse,
    bench_type_conversions
);
criterion_main!(benches);
