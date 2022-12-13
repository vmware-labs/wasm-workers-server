/**
 * Builds a reply to the given request
 */
const reply = () => {
  // Body response
  const body = `<!DOCTYPE html>
  <head>
    <title>Wasm Workers Server</title>
    <meta name="viewport" content="width=device-width,initial-scale=1">
    <meta charset="UTF-8">
    <link rel="stylesheet" href="/water.min.css">
    <link rel="stylesheet" href="/main.css">
  </head>
  <body>
    <main>
      <h1>Hello from Wasm Workers Server 👋</h1>
      <p>
        This is a fixed route. There isn't any parameter here.
      </p>
      <p>Read more about dynamic routes <a href="https://workers.wasmlabs.dev/docs/features/dynamic-routes">in the documentation</a></p>
    </main>
  </body>`;

  return new Response(body);
}

// Subscribe to the Fetch event
addEventListener("fetch", event => {
  return event.respondWith(reply(event.request));
});
