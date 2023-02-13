import sys, json;

class Request:
    def __init__(self, input):
        self.method = input["method"]
        self.url = input["url"]
        self.body = input["body"]
        self.headers = input["headers"]
        self.params = input["params"]

class Response:
    def __init__(self, body):
        self.body = body
        self.status_code = 200
        self.headers = {}
        self.kv = {}

    def to_json(self):
        res = {
            'data': self.body,
            'status': self.status_code,
            'headers': self.headers,
            'kv': self.kv
        }
        return json.dumps(res)
