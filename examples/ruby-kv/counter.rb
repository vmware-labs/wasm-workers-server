CACHE_KEY = "counter";

def worker(request)
  # Prepare the body
  count = Cache.get(CACHE_KEY).to_i || 0
  body = "The counter value is: #{count}"

  # Update the counter
  count += 1
  Cache.set(CACHE_KEY, count)

  # Return the response
  Response.new(body)
end