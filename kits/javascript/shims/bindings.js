// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { TextEncoder } from "@sinonjs/text-encoding";
import { Response } from "./types/response";

(function () {
  const __wws_send_http_request = globalThis.__wws_send_http_request;
  const __wws_console_log = globalThis.__wws_console_log;

  globalThis.fetch = (uri, opts) => {
    let optsWithDefault = {
      method: "GET",
      headers: {},
      body: null,
      ...opts
    };

    if (optsWithDefault.body !== null && typeof optsWithDefault.body !== "string") {
      try {
        optsWithDefault.body = new TextEncoder().encode(optsWithDefault.body);
      } catch (e) {
        return Promise.reject(`There was an error encoding the body: ${e}. Use a String or encode it using TextEncoder.`)
      }
    }

    let result = __wws_send_http_request(uri, optsWithDefault);

    if (result.error === true) {
      return Promise.reject(new Error(`[${result.type}] ${result.message}`));
    } else {
      let response = new Response(result.body, {
        headers: result.headers,
        status: result.status,
      })

      return Promise.resolve(response);
    }
  }

  globalThis.console = {
    error(msg) {
      this.log(msg);
    },
    log(msg) {
      __wws_console_log(msg);
    },
    info(msg) {
      this.log(msg);
    },
    debug(msg) {
      this.log(msg);
    },
    warn(msg) {
      this.log(msg);
    }
  }

  Reflect.deleteProperty(globalThis, "__wws_send_http_request");
  Reflect.deleteProperty(globalThis, "__wws_console_log");
})();
