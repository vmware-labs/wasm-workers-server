package worker

import (
	"bytes"
	"context"
	"encoding/base64"
	"errors"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"os"
	"strings"
	"unicode/utf8"

	"github.com/vmware-labs/wasm-workers-server/kits/go/worker/bindings"

	"github.com/tidwall/gjson"
	"github.com/tidwall/sjson"
)

type ContextKey string

const (
	CacheKey  ContextKey = "CACHE"
	ParamsKey ContextKey = "PARAMS"
)

type input struct {
	Url     string
	Method  string
	Headers map[string]string
	Body    string
}

type output struct {
	Data    string
	Headers map[string]string
	Status  uint16
	Base64  bool

	httpHeader http.Header
}

var (
	cache  map[string]string
	params map[string]string
)

func init() {
	cache = make(map[string]string)
	params = make(map[string]string)
}

// output implements the http.ResponseWriter interface

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

	out, _ := sjson.Set("", "data", o.Data)
	out, _ = sjson.Set(out, "status", o.Status)
	out, _ = sjson.Set(out, "base64", o.Base64)
	out, _ = sjson.SetRaw(out, "headers", "{}")
	out, _ = sjson.SetRaw(out, "kv", "{}")

	for k, v := range o.Headers {
		out, _ = sjson.Set(out, fmt.Sprintf("headers.%s", k), v)
	}

	for k, v := range cache {
		out, _ = sjson.Set(out, fmt.Sprintf("kv.%s", k), v)
	}

	fmt.Println(out)

	return len(o.Data), nil
}

func (o *output) WriteHeader(statusCode int) {
	o.Status = uint16(statusCode)
}

func readInput() (*input, error) {
	stdin, err := io.ReadAll(os.Stdin)
	if err != nil {
		return nil, err
	}

	in := &input{
		Url:     gjson.GetBytes(stdin, "url").String(),
		Method:  gjson.GetBytes(stdin, "method").String(),
		Body:    gjson.GetBytes(stdin, "body").String(),
		Headers: make(map[string]string),
	}

	if gjson.GetBytes(stdin, "headers").Exists() {
		gjson.GetBytes(stdin, "headers").ForEach(func(key, value gjson.Result) bool {
			in.Headers[key.String()] = value.String()
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
		gjson.GetBytes(stdin, "params").ForEach(func(key, value gjson.Result) bool {
			params[key.String()] = value.String()
			return true
		})
	}

	return in, nil
}

func createRequest(in *input) (*http.Request, error) {
	req, err := http.NewRequest(in.Method, in.Url, strings.NewReader(in.Body))
	if err != nil {
		return nil, err
	}

	for k, v := range in.Headers {
		req.Header.Set(k, v)
	}

	req = req.WithContext(context.WithValue(req.Context(), CacheKey, cache))
	req = req.WithContext(context.WithValue(req.Context(), ParamsKey, params))

	return req, nil
}

func getWriterRequest() (*output, *http.Request) {
	in, err := readInput()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	req, err := createRequest(in)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	w := &output{
		Headers: make(map[string]string),
	}

	return w, req
}

func Serve(handler http.Handler) {
	handler.ServeHTTP(getWriterRequest())
}

func ServeFunc(handler http.HandlerFunc) {
	handler(getWriterRequest())
}

func SendHttpRequest(req *http.Request) (*http.Response, error) {
	var method bindings.HttpTypesHttpMethod
	switch req.Method {
	case "GET":
		method = bindings.HttpTypesHttpMethodGet()
	case "POST":
		method = bindings.HttpTypesHttpMethodPost()
	case "PUT":
		method = bindings.HttpTypesHttpMethodPut()
	case "PATCH":
		method = bindings.HttpTypesHttpMethodPatch()
	case "DELETE":
		method = bindings.HttpTypesHttpMethodDelete()
	case "OPTIONS":
		method = bindings.HttpTypesHttpMethodOptions()
	case "HEAD":
		method = bindings.HttpTypesHttpMethodHead()
	default:
		method = bindings.HttpTypesHttpMethodGet()
	}

	// Iterate to get the headers
	headers := make([]bindings.HttpTypesHttpHeader, 0, len(req.Header))
	for key, values := range req.Header {
		for _, value := range values {
			header := bindings.HttpTypesHttpHeader{F0: key, F1: value}
			headers = append(headers, header)
		}
	}

	// Read the body request and convert it
	body := []uint8{}

	if req.Body != nil {
		readBody, err := ioutil.ReadAll(req.Body)
		if err != nil {
			return nil, err
		}
		defer req.Body.Close()

		body = readBody
	}


	// Convert body to []uint8
	bodyBytes := []uint8(body)

	bRequest := bindings.HttpTypesHttpRequest {
		Body: bindings.Some(bodyBytes),
		// Body: bindings.Some([]uint8{}),
		Headers: headers,
		Method: method,
		Params: []bindings.HttpTypesHttpParam{},
		Uri: req.URL.String(),
	}

	result := bindings.HttpSendHttpRequest(bRequest)

	if result.IsOk() {
		response := result.Unwrap()

		// Create a new http.Response
		httpResponse := &http.Response{}
		httpResponse.StatusCode = int(response.Status)

		if response.Body.IsSome() {
			body := response.Body.Unwrap()
			httpResponse.Body = ioutil.NopCloser(bytes.NewReader(body))
		}

		// Set the headers
		httpResponse.Header = make(http.Header)
		for _, header := range response.Headers {
			httpResponse.Header.Add(header.F0, header.F1)
		}

		return httpResponse, nil
	} else {
		err := result.UnwrapErr()

		return nil, errors.New(err.Message)
	}
}
