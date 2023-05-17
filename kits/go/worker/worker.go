package worker

import (
	"context"
	"encoding/base64"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"unicode/utf8"

	"github.com/tidwall/gjson"
)

var cache map[string]string

func init() {
	cache = make(map[string]string)
}

type input struct {
	url     string
	method  string
	headers map[string]string
	body    string
	params  map[string]string
}

type output struct {
	data    string
	headers map[string]string
	status  uint16
	kv      map[string]string
	base64  bool

	httpHeader http.Header
}

func (o *output) Header() http.Header {
	if o.httpHeader == nil {
		o.httpHeader = http.Header{}
	}

	return o.httpHeader
}

func (o *output) Write(data []byte) (int, error) {
	if utf8.Valid(data) {
		o.data = string(data)
	} else {
		o.base64 = true
		o.data = base64.StdEncoding.EncodeToString(data)
	}

	if o.status == 0 {
		o.status = 200
	}

	for k, v := range o.httpHeader {
		o.headers[k] = v[0]
	}

	headersOut := "{"

	for k, v := range o.headers {
		headersOut += fmt.Sprintf(`"%s":"%s",`, k, v)
	}

	headersOut = strings.TrimSuffix(headersOut, ",") + "}"

	kvOut := "{"

	for k, v := range cache {
		kvOut += fmt.Sprintf(`"%s":"%s",`, k, v)
	}

	kvOut = strings.TrimSuffix(kvOut, ",") + "}"

	fmt.Printf("{\"data\":\"%s\",\"headers\":%s,\"status\":%d,\"kv\":%s,\"base64\":%t}",
		strings.ReplaceAll(o.data, "\"", "\\\""), headersOut, o.status, kvOut, o.base64)

	return len(o.data), nil
}

func (o *output) WriteHeader(statusCode int) {
	o.status = uint16(statusCode)
}

func readInput() *input {
	stdin, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	in := &input{
		url:    gjson.GetBytes(stdin, "url").String(),
		method: gjson.GetBytes(stdin, "method").String(),
		body:   gjson.GetBytes(stdin, "body").String(),
	}

	if gjson.GetBytes(stdin, "headers").Exists() {
		in.headers = make(map[string]string)

		gjson.GetBytes(stdin, "headers").ForEach(func(key, value gjson.Result) bool {
			in.headers[key.String()] = value.String()
			return true
		})
	}

	if gjson.GetBytes(stdin, "kv").Exists() {
		gjson.GetBytes(stdin, "kv").ForEach(func(key, value gjson.Result) bool {
			cache[key.String()] = value.String()
			return true
		})
	}

	if gjson.GetBytes(stdin, "params").Exists() {
		in.params = make(map[string]string)

		gjson.GetBytes(stdin, "params").ForEach(func(key, value gjson.Result) bool {
			in.params[key.String()] = value.String()
			return true
		})
	}

	return in
}

func createRequest(in *input) *http.Request {
	req, err := http.NewRequest(in.method, in.url, strings.NewReader(in.body))
	if err != nil {
		panic(err)
	}

	for k, v := range in.headers {
		req.Header.Set(k, v)
	}

	req = req.WithContext(context.WithValue(req.Context(), "CACHE", cache))
	req = req.WithContext(context.WithValue(req.Context(), "PARAMS", in.params))

	return req
}

func getWriterRequest() (*output, *http.Request) {
	in := readInput()
	req := createRequest(in)
	w := &output{
		headers: make(map[string]string),
		kv:      make(map[string]string),
	}

	return w, req
}

func Serve(handler http.Handler) {
	handler.ServeHTTP(getWriterRequest())
}

func ServeFunc(handler http.HandlerFunc) {
	handler(getWriterRequest())
}
