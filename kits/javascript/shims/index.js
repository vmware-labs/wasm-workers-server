// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

// Main entrypoint for the project.

// Import the different external polyfills
import URLSearchParams from "@ungap/url-search-params";
import { TextEncoder, TextDecoder } from "@sinonjs/text-encoding";

// Import all the project types
import "./bindings";
import { Request, Response, Headers, Cache } from "./types";

// Define the globals
globalThis.URLSearchParams = URLSearchParams;
globalThis.TextEncoder = TextEncoder;
globalThis.TextDecoder = TextDecoder;

// Main logic
let handlerFunction;

let addEventListener = (_eventName, handler) => {
  // Store the callback globally
  handlerFunction = handler;
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

  try {
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
        error = `Couldn't process the response from the handler:\n${err}`;
      });
  } catch (err) {
    error = `There was an error running the handler:\n${err}`;
  }
};

// This is the entrypoint for the project.
entrypoint = requestToHandler;

// Set the result
result = {};

// Save errors
error = null
