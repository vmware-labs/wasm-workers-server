CACHE_KEY = "counter"

def worker(request):
    count = Cache.get(CACHE_KEY)

    if count is None:
        count = 0
    else:
        count = int(count)

    # Body response
    body = '''\
        The counter value is: {count}
    '''.format(
        count=count
    )

    # Build a new response
    res = Response(body)

    # Update the counter
    Cache.set(CACHE_KEY, count + 1)

    return res