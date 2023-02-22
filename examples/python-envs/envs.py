import os

def worker(req):
    # Body response
    body = "The environment variable value is: {message}".format(
        message=os.getenv("MESSAGE")
    )

    return Response(body)