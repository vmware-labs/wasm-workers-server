---
title: HTTP Requests (fetch)
sidebar_position: 3
---

:::info

[Available since v1.4](https://github.com/vmware-labs/wasm-workers-server/releases/tag/v1.4.0)

:::

Often times, workers require to access data from an external resource like a website or an API. This feature allows workers to perform HTTP requests to external resources. It follows the capability-based model, so workers cannot perform any HTTP request until you configure the allowed hosts and HTTP methods.

In this configuration, you are allowing a worker to perform `GET` and `POST` HTTP requests to the [{JSON} Placeholder API](https://jsonplaceholder.typicode.com/):

```toml
name = "fetch"
version = "1"

[features]
[features.http_requests]
allowed_methods = ["GET", "POST"]
allowed_hosts = ["jsonplaceholder.typicode.com"]
```

Now, your worker can perform HTTP requests following those rules.

## Send HTTP requests in different languages

Depending on the language, the different kits expose this feature in a different way. The goal is to use a common API to perform HTTP requests in that language. For example, to perform HTTP requests in JavaScript you can use the [`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) API.

Check these guides to perform HTTP requests in the different supported languages:

* [HTTP requests on JavaScript workers](../languages/javascript.md#send-an-http-request-fetch)
* [HTTP requests on Rust workers](../languages/rust.md#send-an-http-request)
* [HTTP requests on Go workers](../languages/go.md#send-an-http-request)

## Language compatibility

| Language | HTTP Requests |
| --- | --- |
| JavaScript | ✅ |
| Rust | ✅ |
| Go | ✅ |
| Ruby | ❌ |
| Python | ❌ |
| Zig | ❌ |
