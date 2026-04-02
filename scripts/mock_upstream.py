#!/usr/bin/env python3
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
import sys

port = int(sys.argv[1]) if len(sys.argv) > 1 else 19091
name = sys.argv[2] if len(sys.argv) > 2 else "mock-a"

class Handler(BaseHTTPRequestHandler):
    def _json(self, code, payload):
        self.send_response(code)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps(payload).encode())

    def do_POST(self):
        length = int(self.headers.get('Content-Length', '0'))
        raw = self.rfile.read(length) if length else b'{}'
        try:
            body = json.loads(raw.decode())
        except Exception:
            body = {}

        if self.path == '/v1/chat/completions':
            model = body.get('model', 'unknown')
            self._json(200, {
                'id': f'chatcmpl-{name}',
                'object': 'chat.completion',
                'created': 0,
                'model': model,
                'choices': [{
                    'index': 0,
                    'message': {
                        'role': 'assistant',
                        'content': f'response from {name} for {model}'
                    },
                    'finish_reason': 'stop'
                }]
            })
        else:
            self._json(404, {'error': {'message': 'not found', 'type': 'not_found'}})

    def log_message(self, format, *args):
        return

HTTPServer(('127.0.0.1', port), Handler).serve_forever()
