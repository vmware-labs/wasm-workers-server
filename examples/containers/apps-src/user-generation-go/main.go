package main

import (
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"strconv"

	"github.com/vmware-labs/wasm-workers-server/kits/go/worker"
)

type User struct {
	FirstName string `json:"first_name"`
	LastName  string `json:"last_name"`
	Username  string `json:"username"`
	Email     string `json:"email"`
}

type ResponseData struct {
	User             User   `json:"user"`
	SomeFileContents string `json:"some_file_contents"`
	GeneratedUsers   uint32 `json:"generated_users"`
}

func main() {
	worker.ServeFunc(func(w http.ResponseWriter, r *http.Request) {
		cache, _ := r.Context().Value(worker.CacheKey).(map[string]string)

		// Create the request
		req, err := http.NewRequest(http.MethodGet, "https://random-data-api.com/api/v2/users", nil)
		if err != nil {
			panic(err)
		}

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

		user := User{}
		err = json.Unmarshal([]byte(resBody), &user)
		if err != nil {
			panic(err)
		}

		fileContents_, err := ioutil.ReadFile("/tmp/file.txt")
		if err != nil {
			panic(err)
		}
		fileContents := string(fileContents_)

		generatedUserCount := uint32(1)
		if count, ok := cache["generated_users_counter"]; ok {
			n, _ := strconv.ParseUint(count, 10, 32)
			generatedUserCount = uint32(n) + 1
		}
		cache["generated_users_counter"] = fmt.Sprintf("%d", generatedUserCount)

		responseData := ResponseData{
			User:             user,
			SomeFileContents: fileContents,
			GeneratedUsers:   generatedUserCount,
		}

		marshaledResponseData, err := json.Marshal(responseData)
		if err != nil {
			panic(err)
		}

		w.Header().Set("x-generated-by", "wasm-workers-server")
		w.Write([]byte(marshaledResponseData))
	})
}
