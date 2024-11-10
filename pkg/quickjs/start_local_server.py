#!/usr/bin/env python3

import http.server
import os
from http.server import SimpleHTTPRequestHandler, HTTPServer

port = 8081
print("Running on port %d" % port)


class NoCacheHTTPRequestHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        # Disable caching by setting headers
        self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
        self.send_header("Pragma", "no-cache")
        self.send_header("Expires", "0")
        super().end_headers()  # Call the parent class's end_headers method to finalize headers

httpd = http.server.HTTPServer(
    ("localhost", port), NoCacheHTTPRequestHandler
)


httpd.serve_forever()
