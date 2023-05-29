package main

import (
	"fmt"
	"net/http"

	"github.com/vmware-labs/wasm-workers-server/kits/go/worker"
)

func main() {
	worker.ServeFunc(func(w http.ResponseWriter, r *http.Request) {
		params, _ := r.Context().Value(worker.ParamsKey).(map[string]string)
		id := "the value is not available"

		if val, ok := params["id"]; ok {
			id = val
		}

		body := fmt.Sprintf("<!DOCTYPE html>"+
			"<head>"+
			"<title>Wasm Workers Server</title>"+
			"<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">"+
			"<meta charset=\"UTF-8\">"+
			"<link rel=\"stylesheet\" href=\"/water.min.css\">"+
			"<link rel=\"stylesheet\" href=\"/main.css\">"+
			"</head>"+
			"<body>"+
			"<main>"+
			"<h1>Hello from Wasm Workers Server 👋</h1>"+
			"<p>"+
			"This is a dynamic route! The <code>[id].wasm</code> worker, written in Go, is replying this URL."+
			"The <code>id</code> parameter value is: <code>%s</code>"+
			"</p>"+
			"<p>Read more about dynamic routes <a href=\"https://workers.wasmlabs.dev/docs/features/dynamic-routes\">in the documentation</a></p>"+
			"</main>"+
			"</body>", id)

		w.Header().Set("x-generated-by", "wasm-workers-server")
		w.Write([]byte(body))
	})
}
