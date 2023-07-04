// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

// Key / Value store from Wasm Workers Server
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

export { Cache };
