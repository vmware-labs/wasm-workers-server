import json
from collections import namedtuple

class User:
    def __init__(self, first_name, last_name, username, email):
        self.first_name, self.last_name, self.username, self.email = first_name, last_name, username, email

    @staticmethod
    def from_json(dict):
      return User(dict['first_name'],
                  dict['last_name'],
                  dict['username'],
                  dict['email'])

def worker(request):
    sample_json = """{
      "first_name": "Tracie",
      "last_name": "Schroeder",
      "username": "tracie.schroeder",
      "email": "tracie.schroeder@email.com",
      "password": "secret"
    }"""
    user = json.loads(sample_json, object_hook=User.from_json)
    return Response(
        json.dumps({
            "user": user.__dict__,
            "some_file_contents": open("/tmp/file.txt").read(),
        }, separators=(',', ':'))
    )
