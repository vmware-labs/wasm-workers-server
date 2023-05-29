---
sidebar_position: 5
---

# Go

Go workers are compiled into a WASI module using [TinyGo](https://tinygo.org/docs/guides/webassembly/). Then, they are loaded by Wasm Workers Server and start processing requests.

## Your first Go worker

Workers can be implemented either as an [http.Handler](https://pkg.go.dev/net/http#Handler) or an [http.HandlerFunc](https://pkg.go.dev/net/http#HandlerFunc).
