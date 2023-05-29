// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

class Headers {
  constructor(initialHeaders) {
    let headers = {};

    // Initialize the headers
    for (const key in initialHeaders) {
      headers[key] = initialHeaders[key];
    }

    this.headers = headers;
  }

  append(key, value) {
    this.headers[key] = value;
    return value;
  }

  set(key, value) {
    this.append(key, value);
    return value;
  }

  delete(key) {
    let dropValue = delete this.headers[key];
    return dropValue;
  }

  get(key) {
    return this.headers[key];
  }

  toJSON() {
    return this.headers;
  }
}

class Request {
  constructor(input) {
    this.url = input.url;
    this.method = input.method;
    this.headers = new Headers(input.headers || {});
    this.body = input.body;
    this.params = input.params || {};
  }

  text() {
    return this.body;
  }
}

class Response {
  constructor(body, options = {}) {
    this.body = body;
    this.headers = new Headers(options.headers || {});
    this.status = options.status || 200;
    this.statusText = options.statusText || "OK";
  }

  static redirect(url, statusCode) {
    let statusText;

    switch (statusCode) {
      case 301:
        statusText = "Moved Permanently";
        break;
      case 302:
        statusText = "Found";
        break;
      case 308:
        statusText = "Permanent Redirect";
        break;
      default:
        // Default to 307
        statusText = "Temporary Redirect";
        break;
    }

    let response = new Response(`Redirecting to ${url}`, {
      status: statusCode,
      statusText,
      headers: {
        Location: url
      }
    });

    return response;
  }

  toString() {
    return this.body;
  }
}

let handlerFunction;

const addEventListener = (_eventName, handler) => {
  // Store the callback globally
  handlerFunction = handler;
};

const Cache = {
  state: {},
  init(state) {
    this.state = state;
  },
  get(key) {
    return this.state[key];
  },
  set(key, value) {
    this.state[key] = value;
  }
};

const requestToHandler = input => {
  const request = new Request(input);
  const event = {
    request,
    response: {},
    respondWith(res) {
      this.response = res;
    }
  };

  Cache.init(input.kv);

  handlerFunction(event);

  // Always convert event.response to a Promise
  Promise.resolve(
    event.response
  ).then(res => {
    // Set the result in the global value
    result = {
      data: res.body,
      headers: res.headers.headers,
      status: res.status,
      kv: Cache.state
    };
  })
  .catch((err) => {
    throw new Error(`Error getting the response in the worker: ${err}`);
  });
};

// This is the entrypoint for the project.
entrypoint = requestToHandler;

// Set the result
result = {};