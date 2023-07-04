// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Headers } from "./headers";
import { TextEncoder, TextDecoder } from "@sinonjs/text-encoding";
import httpStatus from "http-status";

// The response object to return the project response.
// It contains different helpers
class Response {
  constructor(body, options = {}) {
    this.body = body;
    this.headers = new Headers(options.headers || {});
    this.status = options.status || 200;
    this.statusText = options.statusText || httpStatus[this.status];
  }

  static redirect(url, status = 307) {
    return new Response(`Redirecting to ${url}`, {
      status,
      headers: {
        Location: url
      }
    })
  }

  get ok() {
    return this.status >= 200 && this.status < 300;
  }

  defaultEncoding() {
    return "utf-8";
  }

  arrayBuffer() {
    let parsedBody = this.body;

    if (typeof this.body === "string") {
      try {
        // For now, we only consider the String|ArrayBuffer option
        parsedBody = new TextEncoder().encode(this.body);
      } catch (e) {
        return Promise.reject(`There was an error encoding the body: ${e}. Please, use the arrayBuffer() and TextDecoder method instead.`);
      }
    }

    return parsedBody;
  }

  json() {
    let parsedBody = this.body;

    if (typeof this.body !== "string") {
      try {
        // For now, we only consider the String|ArrayBuffer option
        parsedBody = new TextDecoder(this.defaultEncoding()).decode(this.body);
      } catch (e) {
        return Promise.reject(`There was an error decoding the body: ${e}. Please, use the arrayBuffer() and TextDecoder method instead.`);
      }
    }

    try {
      return Promise.resolve(JSON.parse(parsedBody));
    } catch (e) {
      return Promise.reject(`The body is not a valid JSON: ${e}`);
    }
  }

  text() {
    let parsedBody = this.body;

    if (typeof this.body !== "string") {
      try {
        // For now, we only consider the String|ArrayBuffer option
        parsedBody = new TextDecoder(this.defaultEncoding()).decode(this.body);
      } catch (e) {
        return Promise.reject(`There was an error decoding the body: ${e}. Please, use the arrayBuffer() and TextDecoder method instead.`);
      }
    }

    return parsedBody;
  }

  toString() {
    return this.body;
  }
}

export { Response };
