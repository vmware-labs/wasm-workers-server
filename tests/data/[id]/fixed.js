/**
 * Builds a reply to the given request
 */
const reply = () => {
  // Build a new response
  let response = new Response("Hello from a fixed route in a parametrized subroute");

  return response;
}

// Subscribe to the Fetch event
addEventListener("fetch", event => {
  return event.respondWith(reply(event.request));
});
