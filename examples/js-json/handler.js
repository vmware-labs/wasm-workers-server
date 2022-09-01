const JSON_CONTENT_TYPE = "application/json;charset=UTF-8";

/**
 * Builds a reply to the given request. In this case,
 * this reply method returns a JSON response
 */
const reply = (request) => {
  if (request.method != "GET") {
    // Don't allow other methods.
    // Here you can see how to return a custom status
    const errorResponse = JSON.stringify({
      success: false,
      error: "Method not allowed"
    });

    return new Response(errorResponse, {
      status: 405,
      headers: {
        "content-type": JSON_CONTENT_TYPE
      }
    });
  }

  // Body response
  const body = JSON.stringify({
    success: true,
    message: "Hello from Wasm!"
  });

  // Build a new response
  let response = new Response(body, {
    headers: {
      "content-type": JSON_CONTENT_TYPE,
      "x-generated-by": "wasm-workers-server"
    }
  });

  return response;
}

// Subscribe to the Fetch event
addEventListener("fetch", event => {
  return event.respondWith(reply(event.request));
});