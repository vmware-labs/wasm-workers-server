# Read an environment variable in Ruby
def worker(request)
    img = IO.read("/src/images/ruby.svg", mode: "rb")

    res = Response.new(img)
    res.headers["Content-Type"] = "image/svg+xml"

    res
end