package worker

import (
	"encoding/base64"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"unicode/utf8"

	"github.com/tidwall/gjson"
)

type input struct {
	Url     string
	Method  string
	Headers map[string]string
	Body    string
	Kv      map[string]string
	Params  map[string]string
}

type output struct {
	Data    string
	Headers map[string]string
	Status  uint16
	Kv      map[string]string
	Base64  bool

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
		o.Data = string(data)
	} else {
		o.Base64 = true
		o.Data = base64.StdEncoding.EncodeToString(data)
	}

	if o.Status == 0 {
		o.Status = 200
	}

	for k, v := range o.httpHeader {
		o.Headers[k] = v[0]
	}

	headersOut := "{"

	for k, v := range o.Headers {
		headersOut += fmt.Sprintf(`"%s":"%s",`, k, v)
	}

	headersOut = strings.TrimSuffix(headersOut, ",") + "}"

	kvOut := "{"

	for k, v := range o.Kv {
		kvOut += fmt.Sprintf(`"%s":"%s",`, k, v)
	}

	kvOut = strings.TrimSuffix(kvOut, ",") + "}"

	fmt.Printf("{\"data\":\"%s\",\"headers\":%s,\"status\":%d,\"kv\":%s,\"base64\":%t}",
		o.Data, headersOut, o.Status, kvOut, o.Base64)

	return len(o.Data), nil
}

func (o *output) WriteHeader(statusCode int) {
	o.Status = uint16(statusCode)
}

func readInput() *input {
	stdin, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	in := &input{
		Url:    gjson.GetBytes(stdin, "url").String(),
		Method: gjson.GetBytes(stdin, "method").String(),
		Body:   gjson.GetBytes(stdin, "body").String(),
	}

	if gjson.GetBytes(stdin, "headers").Exists() {
		in.Headers = make(map[string]string)

		gjson.GetBytes(stdin, "headers").ForEach(func(key, value gjson.Result) bool {
			in.Headers[key.String()] = value.String()
			return true
		})
	}

	if gjson.GetBytes(stdin, "kv").Exists() {
		in.Kv = make(map[string]string)

		gjson.GetBytes(stdin, "kv").ForEach(func(key, value gjson.Result) bool {
			in.Headers[key.String()] = value.String()
			return true
		})
	}

	if gjson.GetBytes(stdin, "params").Exists() {
		in.Params = make(map[string]string)

		gjson.GetBytes(stdin, "params").ForEach(func(key, value gjson.Result) bool {
			in.Headers[key.String()] = value.String()
			return true
		})
	}

	return in
}

func createRequest(in *input) *http.Request {
	req, err := http.NewRequest(in.Method, in.Url, strings.NewReader(in.Body))
	if err != nil {
		panic(err)
	}

	for k, v := range in.Headers {
		req.Header.Set(k, v)
	}

	return req
}

func getWriterRequest() (*output, *http.Request) {
	in := readInput()
	req := createRequest(in)
	w := &output{
		Headers: make(map[string]string),
		Kv:      in.Kv,
	}

	return w, req
}

func Serve(handler http.Handler) {
	handler.ServeHTTP(getWriterRequest())
}

func ServeFunc(handler http.HandlerFunc) {
	handler(getWriterRequest())
}
