/**
 * Builds a reply to the given request. In this case,
 * this reply method returns a redirect
 */
const reply = (request) => {
  if (request.method != "GET") {
    // Don't allow other methods.
    // Here you can see how to return a custom status
    return new Response("Method not allowed", {
      status: 405,
    });
  }

  // Build a redirect response
  let response = Response.redirect("https://example.com", 301);

  return response;
}

// Subscribe to the Fetch event
addEventListener("fetch", event => {
  return event.respondWith(reply(event.request));
});