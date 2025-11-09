#!/usr/bin/env python3
"""
Simple HTTP server for benchmarking

Usage:
    python server.py
"""

from http.server import HTTPServer, BaseHTTPRequestHandler
import json

# Pre-generate response data
RESPONSES = {
    '1k': b'x' * 1024,  # 1 KB
    '20k': b'x' * (20 * 1024),  # 20 KB
    '200k': b'x' * (200 * 1024),  # 200 KB
}

class BenchmarkHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        path = self.path.lstrip('/')

        # Serve different response sizes
        if path in RESPONSES:
            self.send_response(200)
            self.send_header('Content-Type', 'text/plain')
            self.send_header('Content-Length', str(len(RESPONSES[path])))
            self.end_headers()
            self.wfile.write(RESPONSES[path])
        elif path == '':
            # Root path - serve info
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            info = {
                'message': 'Benchmark server running',
                'endpoints': list(RESPONSES.keys()),
            }
            self.wfile.write(json.dumps(info, indent=2).encode())
        else:
            self.send_response(404)
            self.send_header('Content-Type', 'text/plain')
            self.end_headers()
            self.wfile.write(b'Not Found')

    def log_message(self, format, *args):
        # Suppress logging for benchmarks
        pass

def run_server(port=8000):
    server_address = ('', port)
    httpd = HTTPServer(server_address, BenchmarkHandler)
    print(f"Starting benchmark server on port {port}...")
    print(f"Available endpoints: {list(RESPONSES.keys())}")
    print("Press Ctrl+C to stop")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nServer stopped.")

if __name__ == '__main__':
    run_server()
