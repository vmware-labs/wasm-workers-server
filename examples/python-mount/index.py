# Read a mounted file and return it
def worker(request):
    s = ""
    with open("/src/assets/index.html") as f:
        s = f.read()

    return Response(s)