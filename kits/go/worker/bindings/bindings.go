package bindings

// #include "bindings.h"
import "C"

import "unsafe"

// http-types
type HttpTypesUri = string
type HttpTypesHttpStatus = uint16
type HttpTypesHttpParam struct {
  F0 string
  F1 string
}

type HttpTypesHttpParams = HttpTypesHttpParam
type HttpTypesHttpMethodKind int

const (
HttpTypesHttpMethodKindGet HttpTypesHttpMethodKind = iota
HttpTypesHttpMethodKindPost
HttpTypesHttpMethodKindPut
HttpTypesHttpMethodKindPatch
HttpTypesHttpMethodKindDelete
HttpTypesHttpMethodKindOptions
HttpTypesHttpMethodKindHead
)

type HttpTypesHttpMethod struct {
  kind HttpTypesHttpMethodKind
}

func (n HttpTypesHttpMethod) Kind() HttpTypesHttpMethodKind {
  return n.kind
}

func HttpTypesHttpMethodGet() HttpTypesHttpMethod{
  return HttpTypesHttpMethod{kind: HttpTypesHttpMethodKindGet}
}

func HttpTypesHttpMethodPost() HttpTypesHttpMethod{
  return HttpTypesHttpMethod{kind: HttpTypesHttpMethodKindPost}
}

func HttpTypesHttpMethodPut() HttpTypesHttpMethod{
  return HttpTypesHttpMethod{kind: HttpTypesHttpMethodKindPut}
}

func HttpTypesHttpMethodPatch() HttpTypesHttpMethod{
  return HttpTypesHttpMethod{kind: HttpTypesHttpMethodKindPatch}
}

func HttpTypesHttpMethodDelete() HttpTypesHttpMethod{
  return HttpTypesHttpMethod{kind: HttpTypesHttpMethodKindDelete}
}

func HttpTypesHttpMethodOptions() HttpTypesHttpMethod{
  return HttpTypesHttpMethod{kind: HttpTypesHttpMethodKindOptions}
}

func HttpTypesHttpMethodHead() HttpTypesHttpMethod{
  return HttpTypesHttpMethod{kind: HttpTypesHttpMethodKindHead}
}

type HttpTypesHttpHeader struct {
  F0 string
  F1 string
}

type HttpTypesHttpHeaders = HttpTypesHttpHeader
type HttpTypesHttpErrorKind int

const (
HttpTypesHttpErrorKindInvalidRequest HttpTypesHttpErrorKind = iota
HttpTypesHttpErrorKindInvalidRequestBody
HttpTypesHttpErrorKindInvalidResponseBody
HttpTypesHttpErrorKindNotAllowed
HttpTypesHttpErrorKindInternalError
HttpTypesHttpErrorKindTimeout
HttpTypesHttpErrorKindRedirectLoop
)

type HttpTypesHttpError struct {
  kind HttpTypesHttpErrorKind
}

func (n HttpTypesHttpError) Kind() HttpTypesHttpErrorKind {
  return n.kind
}

func HttpTypesHttpErrorInvalidRequest() HttpTypesHttpError{
  return HttpTypesHttpError{kind: HttpTypesHttpErrorKindInvalidRequest}
}

func HttpTypesHttpErrorInvalidRequestBody() HttpTypesHttpError{
  return HttpTypesHttpError{kind: HttpTypesHttpErrorKindInvalidRequestBody}
}

func HttpTypesHttpErrorInvalidResponseBody() HttpTypesHttpError{
  return HttpTypesHttpError{kind: HttpTypesHttpErrorKindInvalidResponseBody}
}

func HttpTypesHttpErrorNotAllowed() HttpTypesHttpError{
  return HttpTypesHttpError{kind: HttpTypesHttpErrorKindNotAllowed}
}

func HttpTypesHttpErrorInternalError() HttpTypesHttpError{
  return HttpTypesHttpError{kind: HttpTypesHttpErrorKindInternalError}
}

func HttpTypesHttpErrorTimeout() HttpTypesHttpError{
  return HttpTypesHttpError{kind: HttpTypesHttpErrorKindTimeout}
}

func HttpTypesHttpErrorRedirectLoop() HttpTypesHttpError{
  return HttpTypesHttpError{kind: HttpTypesHttpErrorKindRedirectLoop}
}

type HttpTypesHttpRequestError struct {
  Error HttpTypesHttpError
  Message string
}

type HttpTypesHttpBody = uint8
type HttpTypesHttpResponse struct {
  Body Option[[]uint8]
  Headers []HttpTypesHttpHeader
  Status uint16
}

type HttpTypesHttpRequest struct {
  Body Option[[]uint8]
  Headers []HttpTypesHttpHeader
  Method HttpTypesHttpMethod
  Params []HttpTypesHttpParam
  Uri string
}

// http
type HttpHttpRequest = HttpTypesHttpRequest
type HttpHttpResponse = HttpTypesHttpResponse
type HttpHttpRequestError = HttpTypesHttpRequestError
func HttpSendHttpRequest(request HttpTypesHttpRequest) Result[HttpTypesHttpResponse, HttpTypesHttpRequestError] {
  var lower_request C.http_types_http_request_t
  var lower_request_val C.http_types_http_request_t
  var lower_request_val_body C.bindings_option_http_body_t
  if request.Body.IsSome() {
    var lower_request_val_body_val C.http_types_http_body_t
    if len(request.Body.Unwrap()) == 0 {
      lower_request_val_body_val.ptr = nil
      lower_request_val_body_val.len = 0
    } else {
      var empty_lower_request_val_body_val C.uint8_t
      lower_request_val_body_val.ptr = (*C.uint8_t)(C.malloc(C.size_t(len(request.Body.Unwrap())) * C.size_t(unsafe.Sizeof(empty_lower_request_val_body_val))))
      lower_request_val_body_val.len = C.size_t(len(request.Body.Unwrap()))
      for lower_request_val_body_val_i := range request.Body.Unwrap() {
        lower_request_val_body_val_ptr := (*C.uint8_t)(unsafe.Pointer(uintptr(unsafe.Pointer(lower_request_val_body_val.ptr)) +
        uintptr(lower_request_val_body_val_i)*unsafe.Sizeof(empty_lower_request_val_body_val)))
        lower_request_val_body_val_ptr_value := C.uint8_t(request.Body.Unwrap()[lower_request_val_body_val_i])
        *lower_request_val_body_val_ptr = lower_request_val_body_val_ptr_value
      }
    }
    lower_request_val_body.val = lower_request_val_body_val
    lower_request_val_body.is_some = true
  }
  lower_request_val.body = lower_request_val_body
  var lower_request_val_headers C.http_types_http_headers_t
  if len(request.Headers) == 0 {
    lower_request_val_headers.ptr = nil
    lower_request_val_headers.len = 0
  } else {
    var empty_lower_request_val_headers C.http_types_http_header_t
    lower_request_val_headers.ptr = (*C.http_types_http_header_t)(C.malloc(C.size_t(len(request.Headers)) * C.size_t(unsafe.Sizeof(empty_lower_request_val_headers))))
    lower_request_val_headers.len = C.size_t(len(request.Headers))
    for lower_request_val_headers_i := range request.Headers {
      lower_request_val_headers_ptr := (*C.http_types_http_header_t)(unsafe.Pointer(uintptr(unsafe.Pointer(lower_request_val_headers.ptr)) +
      uintptr(lower_request_val_headers_i)*unsafe.Sizeof(empty_lower_request_val_headers)))
      var lower_request_val_headers_ptr_value C.http_types_http_header_t
      var lower_request_val_headers_ptr_value_f0 C.bindings_string_t
      
      lower_request_val_headers_ptr_value_f0.ptr = C.CString(request.Headers[lower_request_val_headers_i].F0)
      lower_request_val_headers_ptr_value_f0.len = C.size_t(len(request.Headers[lower_request_val_headers_i].F0))
      lower_request_val_headers_ptr_value.f0 = lower_request_val_headers_ptr_value_f0
      var lower_request_val_headers_ptr_value_f1 C.bindings_string_t
      
      lower_request_val_headers_ptr_value_f1.ptr = C.CString(request.Headers[lower_request_val_headers_i].F1)
      lower_request_val_headers_ptr_value_f1.len = C.size_t(len(request.Headers[lower_request_val_headers_i].F1))
      lower_request_val_headers_ptr_value.f1 = lower_request_val_headers_ptr_value_f1
      *lower_request_val_headers_ptr = lower_request_val_headers_ptr_value
    }
  }
  lower_request_val.headers = lower_request_val_headers
  var lower_request_val_method C.http_types_http_method_t
  if request.Method.Kind() == HttpTypesHttpMethodKindGet {
    lower_request_val_method = 0
  }
  if request.Method.Kind() == HttpTypesHttpMethodKindPost {
    lower_request_val_method = 1
  }
  if request.Method.Kind() == HttpTypesHttpMethodKindPut {
    lower_request_val_method = 2
  }
  if request.Method.Kind() == HttpTypesHttpMethodKindPatch {
    lower_request_val_method = 3
  }
  if request.Method.Kind() == HttpTypesHttpMethodKindDelete {
    lower_request_val_method = 4
  }
  if request.Method.Kind() == HttpTypesHttpMethodKindOptions {
    lower_request_val_method = 5
  }
  if request.Method.Kind() == HttpTypesHttpMethodKindHead {
    lower_request_val_method = 6
  }
  lower_request_val.method = lower_request_val_method
  var lower_request_val_params C.http_types_http_params_t
  if len(request.Params) == 0 {
    lower_request_val_params.ptr = nil
    lower_request_val_params.len = 0
  } else {
    var empty_lower_request_val_params C.http_types_http_param_t
    lower_request_val_params.ptr = (*C.http_types_http_param_t)(C.malloc(C.size_t(len(request.Params)) * C.size_t(unsafe.Sizeof(empty_lower_request_val_params))))
    lower_request_val_params.len = C.size_t(len(request.Params))
    for lower_request_val_params_i := range request.Params {
      lower_request_val_params_ptr := (*C.http_types_http_param_t)(unsafe.Pointer(uintptr(unsafe.Pointer(lower_request_val_params.ptr)) +
      uintptr(lower_request_val_params_i)*unsafe.Sizeof(empty_lower_request_val_params)))
      var lower_request_val_params_ptr_value C.http_types_http_param_t
      var lower_request_val_params_ptr_value_f0 C.bindings_string_t
      
      lower_request_val_params_ptr_value_f0.ptr = C.CString(request.Params[lower_request_val_params_i].F0)
      lower_request_val_params_ptr_value_f0.len = C.size_t(len(request.Params[lower_request_val_params_i].F0))
      lower_request_val_params_ptr_value.f0 = lower_request_val_params_ptr_value_f0
      var lower_request_val_params_ptr_value_f1 C.bindings_string_t
      
      lower_request_val_params_ptr_value_f1.ptr = C.CString(request.Params[lower_request_val_params_i].F1)
      lower_request_val_params_ptr_value_f1.len = C.size_t(len(request.Params[lower_request_val_params_i].F1))
      lower_request_val_params_ptr_value.f1 = lower_request_val_params_ptr_value_f1
      *lower_request_val_params_ptr = lower_request_val_params_ptr_value
    }
  }
  lower_request_val.params = lower_request_val_params
  var lower_request_val_uri C.bindings_string_t
  var lower_request_val_uri_val C.bindings_string_t
  
  lower_request_val_uri_val.ptr = C.CString(request.Uri)
  lower_request_val_uri_val.len = C.size_t(len(request.Uri))
  lower_request_val_uri = lower_request_val_uri_val
  lower_request_val.uri = lower_request_val_uri
  lower_request = lower_request_val
  defer C.http_interface_http_request_free(&lower_request)
  var ret C.bindings_result_http_response_http_request_error_t
  C.http_send_http_request(&lower_request, &ret)
  var lift_ret Result[HttpTypesHttpResponse, HttpTypesHttpRequestError]
  if ret.is_err {
    lift_ret_ptr := *(*C.http_interface_http_request_error_t)(unsafe.Pointer(&ret.val))
    var lift_ret_val HttpTypesHttpRequestError
    var lift_ret_val_val HttpTypesHttpRequestError
    var lift_ret_val_val_Error HttpTypesHttpError
    if lift_ret_ptr.error == 0 {
      lift_ret_val_val_Error = HttpTypesHttpErrorInvalidRequest()
    }
    if lift_ret_ptr.error == 1 {
      lift_ret_val_val_Error = HttpTypesHttpErrorInvalidRequestBody()
    }
    if lift_ret_ptr.error == 2 {
      lift_ret_val_val_Error = HttpTypesHttpErrorInvalidResponseBody()
    }
    if lift_ret_ptr.error == 3 {
      lift_ret_val_val_Error = HttpTypesHttpErrorNotAllowed()
    }
    if lift_ret_ptr.error == 4 {
      lift_ret_val_val_Error = HttpTypesHttpErrorInternalError()
    }
    if lift_ret_ptr.error == 5 {
      lift_ret_val_val_Error = HttpTypesHttpErrorTimeout()
    }
    if lift_ret_ptr.error == 6 {
      lift_ret_val_val_Error = HttpTypesHttpErrorRedirectLoop()
    }
    lift_ret_val_val.Error = lift_ret_val_val_Error
    var lift_ret_val_val_Message string
    lift_ret_val_val_Message = C.GoStringN(lift_ret_ptr.message.ptr, C.int(lift_ret_ptr.message.len))
    lift_ret_val_val.Message = lift_ret_val_val_Message
    lift_ret_val = lift_ret_val_val
    lift_ret.SetErr(lift_ret_val)
  } else {
    lift_ret_ptr := *(*C.http_interface_http_response_t)(unsafe.Pointer(&ret.val))
    var lift_ret_val HttpTypesHttpResponse
    var lift_ret_val_val HttpTypesHttpResponse
    var lift_ret_val_val_Body Option[[]uint8]
    if lift_ret_ptr.body.is_some {
      var lift_ret_val_val_Body_val []uint8
      lift_ret_val_val_Body_val = make([]uint8, lift_ret_ptr.body.val.len)
      if lift_ret_ptr.body.val.len > 0 {
        for lift_ret_val_val_Body_val_i := 0; lift_ret_val_val_Body_val_i < int(lift_ret_ptr.body.val.len); lift_ret_val_val_Body_val_i++ {
          var empty_lift_ret_val_val_Body_val C.uint8_t
          lift_ret_val_val_Body_val_ptr := *(*C.uint8_t)(unsafe.Pointer(uintptr(unsafe.Pointer(lift_ret_ptr.body.val.ptr)) +
          uintptr(lift_ret_val_val_Body_val_i)*unsafe.Sizeof(empty_lift_ret_val_val_Body_val)))
          var list_lift_ret_val_val_Body_val uint8
          list_lift_ret_val_val_Body_val = uint8(lift_ret_val_val_Body_val_ptr)
          lift_ret_val_val_Body_val[lift_ret_val_val_Body_val_i] = list_lift_ret_val_val_Body_val
        }
      }
      lift_ret_val_val_Body.Set(lift_ret_val_val_Body_val)
    } else {
      lift_ret_val_val_Body.Unset()
    }
    lift_ret_val_val.Body = lift_ret_val_val_Body
    var lift_ret_val_val_Headers []HttpTypesHttpHeader
    lift_ret_val_val_Headers = make([]HttpTypesHttpHeader, lift_ret_ptr.headers.len)
    if lift_ret_ptr.headers.len > 0 {
      for lift_ret_val_val_Headers_i := 0; lift_ret_val_val_Headers_i < int(lift_ret_ptr.headers.len); lift_ret_val_val_Headers_i++ {
        var empty_lift_ret_val_val_Headers C.http_types_http_header_t
        lift_ret_val_val_Headers_ptr := *(*C.http_types_http_header_t)(unsafe.Pointer(uintptr(unsafe.Pointer(lift_ret_ptr.headers.ptr)) +
        uintptr(lift_ret_val_val_Headers_i)*unsafe.Sizeof(empty_lift_ret_val_val_Headers)))
        var list_lift_ret_val_val_Headers HttpTypesHttpHeader
        var list_lift_ret_val_val_Headers_F0 string
        list_lift_ret_val_val_Headers_F0 = C.GoStringN(lift_ret_val_val_Headers_ptr.f0.ptr, C.int(lift_ret_val_val_Headers_ptr.f0.len))
        list_lift_ret_val_val_Headers.F0 = list_lift_ret_val_val_Headers_F0
        var list_lift_ret_val_val_Headers_F1 string
        list_lift_ret_val_val_Headers_F1 = C.GoStringN(lift_ret_val_val_Headers_ptr.f1.ptr, C.int(lift_ret_val_val_Headers_ptr.f1.len))
        list_lift_ret_val_val_Headers.F1 = list_lift_ret_val_val_Headers_F1
        lift_ret_val_val_Headers[lift_ret_val_val_Headers_i] = list_lift_ret_val_val_Headers
      }
    }
    lift_ret_val_val.Headers = lift_ret_val_val_Headers
    var lift_ret_val_val_Status uint16
    var lift_ret_val_val_Status_val uint16
    lift_ret_val_val_Status_val = uint16(lift_ret_ptr.status)
    lift_ret_val_val_Status = lift_ret_val_val_Status_val
    lift_ret_val_val.Status = lift_ret_val_val_Status
    lift_ret_val = lift_ret_val_val
    lift_ret.Set(lift_ret_val)
  }
  return lift_ret
}

