// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

// Define a set of headers. It includes multiple method
// to access and edit them.
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

export { Headers };
