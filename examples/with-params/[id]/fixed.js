/**
 * Builds a reply to the given request
 */
const reply = (req) => {
  // Build a new response
  let response = new Response(`Hello from a fixed route in a parametrized subroute: ${req.params.id}`);

  return response;
}

// Subscribe to the Fetch event
addEventListener("fetch", event => {
  return event.respondWith(reply(event.request));
});
