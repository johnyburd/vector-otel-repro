from http.server import BaseHTTPRequestHandler, HTTPServer

class RequestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self._log_request()
        self._send_ok_response()

    def do_POST(self):
        self._log_request(with_body=True)
        self._send_ok_response()

    def _log_request(self, with_body=False):
        print(f"Path: {self.path}")
        print("Headers:")
        for key, value in self.headers.items():
            print(f"  {key}: {value}")
        
        if with_body:
            content_length = int(self.headers.get('Content-Length', 0))
            body = self.rfile.read(content_length)
            print(f"Body size: {len(body)} bytes")

    def _send_ok_response(self):
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b'OK')

def run(server_class=HTTPServer, handler_class=RequestHandler, port=4318):
    server_address = ('', port)
    httpd = server_class(server_address, handler_class)
    print(f"Starting server on port {port}...")
    httpd.serve_forever()

if __name__ == '__main__':
    run()
