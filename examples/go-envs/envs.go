package main

import (
	"fmt"
	"net/http"
	"os"

	"github.com/vmware-labs/wasm-workers-server/kits/go/worker"
)

func main() {
	worker.ServeFunc(func(w http.ResponseWriter, r *http.Request) {
		body := fmt.Sprintf("The environment variable value is: %s", os.Getenv("MESSAGE"))

		w.Header().Set("x-generated-by", "wasm-workers-server")
		w.Write([]byte(body))
	})
}
