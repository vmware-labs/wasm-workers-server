import sys, json, base64

class Cache:
    store = {}

    @classmethod
    def init(cls, kv):
        cls.store = kv

    @classmethod
    def dump(cls):
        return cls.store

    @classmethod
    def get(cls, key):
        return cls.store.get(key)

    @classmethod
    def set(cls, key, value):
        cls.store[key] = str(value)

class Request:
    def __init__(self, input):
        self.method = input["method"]
        self.url = input["url"]
        self.body = input["body"]
        self.headers = input["headers"]
        self.params = input["params"]

        # Init the cache
        Cache.init(input["kv"])

class Response:
    def __init__(self, body):
        self.body = body
        self.status_code = 200
        self.headers = {}

    def to_json(self):
        content = self.body
        encoded = False

        if type(self.body) is bytes:
            content = base64.b64encode(self.body).decode('utf-8')
            encoded = True

        res = {
            'data': content,
            'status': self.status_code,
            'headers': self.headers,
            'kv': Cache.dump(),
            'base64': encoded
        }

        return json.dumps(res)
