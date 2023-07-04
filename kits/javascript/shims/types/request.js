// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Headers } from "./headers";

// A request that comes from Wasm Workers Server.
// It contains all the required data.
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

export { Request };
