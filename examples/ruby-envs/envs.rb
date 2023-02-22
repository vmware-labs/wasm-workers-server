# Read an environment variable in Ruby
def worker(request)
    Response.new(
        "The environment variable value is: #{ENV.fetch('MESSAGE')}"
    )
end