---
sidebar_position: 1
---

# Key / Value Store

Wasm Workers Server integrates an in-memory [Key / Value (K/V) store](https://en.wikipedia.org/wiki/Key%E2%80%93value_database). This K/V store allows the server to read and write data from the different workers.

For now, the data is only stored in memory and cleaned up on every restart.

## How it works?

The K/V store follows the same snapshot approach as [Requests / Responses](../get-started/how-it-works.md#how-it-works) data. On every request, the worker receives a snapshot of the K/V status for the configured namespace.

The worker may access all the data and perform changes over it. Then, a new K/V status is returned and the internal status is overriden.

### Add a K/V to a worker

* [Add a K/V store to JavaScript workers](../languages/javascript.md#add-a-key--value-store)
* [Add a K/V store to Rust workers](../languages/rust.md#add-a-key--value-store)
* [Add a K/V store to Python workers](../languages/python.md#add-a-key--value-store)
* [Add a K/V store to Ruby workers](../languages/ruby.md#add-a-key--value-store)
* [Add a K/V store to Go workers](../languages/go.md#add-a-key--value-store)

## Limitations

A known limitation of the snapshot approach is the data override when concurrent requests are writing to the same namespace. In the future, we will implement a consolidation mechanism or a different store type for applications that require to write intensively.