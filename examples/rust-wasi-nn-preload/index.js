/**
 * Builds a reply to the given request
 */
const reply = (request) => {
  if (request.method != "GET") {
    // Don't allow other methods.
    // Here you can see how to return a custom status
    return new Response("Method not allowed", {
      status: 405
    });
  }

  // Body response
  const body = `<!DOCTYPE html>
<head>
  <title>Wasm Workers Server</title>
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <meta charset="UTF-8">
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">
  <link rel="stylesheet" href="/main.css">
</head>
<body>
  <main>
    <h1>Hello from Wasm Workers Server 👋</h1>
    <p>Upload an image to detect the content!</p>
    <p><input id="file" type="file" /></p>
    <div class="image">
      <img id="image" />
    </div>
    <div class="results">
      <div class="result" id="result-0">
        <div class="result_progress"></div>
        <div class="result_label">-</div>
      </div>
      <div class="result" id="result-1">
        <div class="result_progress"></div>
        <div class="result_label">-</div>
      </div>
      <div class="result" id="result-2">
        <div class="result_progress"></div>
        <div class="result_label">-</div>
      </div>
      <div class="result" id="result-3">
        <div class="result_progress"></div>
        <div class="result_label">-</div>
      </div>
      <div class="result" id="result-4">
        <div class="result_progress"></div>
        <div class="result_label">-</div>
      </div>
    </div>
  </main>
  <script src="/main.js"></script>
</body>`;

  // Build a new response
  let response = new Response(body);

  // Add a new header
  response.headers.set("x-generated-by", "wasm-workers-server");

  return response;
}

// Subscribe to the Fetch event
addEventListener("fetch", event => {
  return event.respondWith(reply(event.request));
});
