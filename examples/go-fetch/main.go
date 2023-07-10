package main

import (
	"bytes"
	"io"
	"net/http"

	"github.com/vmware-labs/wasm-workers-server/kits/go/worker"

	"github.com/tidwall/sjson"
)

func main() {
	worker.ServeFunc(func(w http.ResponseWriter, r *http.Request) {
		// Build a JSON body
		body, _ := sjson.Set("", "title", "New POST!")
		body, _ = sjson.Set(body, "body", "This is the body")
		body, _ = sjson.Set(body, "userId", 1)

		url := "https://jsonplaceholder.typicode.com/posts"

		// Create the request
		req, err := http.NewRequest(http.MethodPost, url, bytes.NewBufferString(body))
		if err != nil {
			panic(err)
		}
		req.Header.Set("Content-Type", "application/json")

		res, err := worker.SendHttpRequest(req)
		if err != nil {
			panic(err)
		}

		// Read the response
		resBody, err := io.ReadAll(res.Body)
		if err != nil {
			panic(err)
		}
		res.Body.Close()

		w.Header().Set("x-generated-by", "wasm-workers-server")
		w.Write([]byte(resBody))
	})
}
