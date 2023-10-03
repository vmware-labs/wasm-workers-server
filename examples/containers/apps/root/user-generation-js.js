const setCache = (key, data) => Cache.set(key, data);
const getCache = key => Cache.get(key);

const reply = async (request) => {
  try {
    let res = await fetch("https://random-data-api.com/api/v2/users");
    let res_json = await res.json();

    let generated_users_counter = getCache("generated_users_counter");
    if (!generated_users_counter) {
      generated_users_counter = 1;
    } else {
      generated_users_counter = parseInt(generated_users_counter, 10) + 1;
    }
    setCache("generated_users_counter", generated_users_counter.toString());

    return new Response(
      JSON.stringify({
        "user": {
          "first_name": res_json.first_name,
          "last_name": res_json.last_name,
          "username": res_json.username,
          "email": res_json.email
        },
        "generated_users": generated_users_counter
      }),
      {
        "headers": {
          "x-generated-by": "wasm-workers-server"
        }
      }
    );
  } catch (e) {
    return new Response(JSON.stringify({ "error": e.toString() }), { "status": 500 });
  }
}

// Subscribe to the Fetch event
addEventListener("fetch", event => {
  return event.respondWith(reply(event.request));
});
