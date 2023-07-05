use ::http::{Method, Request, Response};

// Load bindings from WIT file.
wit_bindgen_rust::import!({paths: ["../../wit/core/http.wit"]});

use self::http::{
    send_http_request as host_send_http_request, HttpMethod, HttpRequest, HttpResponse,
};

pub use self::http::HttpRequestError;

impl From<HttpResponse> for Response<Vec<u8>> {
    fn from(value: HttpResponse) -> Self {
        let mut builder = Response::builder().status(value.status);

        for (key, value) in value.headers.iter() {
            builder = builder.header(key, value);
        }

        match value.body {
            Some(data) => builder.body(data).unwrap(),
            None => builder.body(Vec::new()).unwrap(),
        }
    }
}

pub fn send_http_request<T>(req: Request<T>) -> Result<Response<Vec<u8>>, HttpRequestError>
where
    T: Into<Vec<u8>>,
{
    let method = match *req.method() {
        Method::GET => HttpMethod::Get,
        Method::POST => HttpMethod::Get,
        Method::PUT => HttpMethod::Get,
        Method::PATCH => HttpMethod::Get,
        Method::DELETE => HttpMethod::Get,
        Method::OPTIONS => HttpMethod::Get,
        Method::HEAD => HttpMethod::Get,
        _ => HttpMethod::Get,
    };

    let mut parsed_headers: Vec<(String, String)> = Vec::new();

    for (key, value) in req.headers().iter() {
        if let Ok(value) = value.to_str() {
            parsed_headers.push((key.to_string(), value.to_string()))
        }
    }

    let headers_slice: Vec<(&str, &str)> = parsed_headers
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect::<Vec<(&str, &str)>>();

    let uri = req.uri().to_string();
    let body: Vec<u8> = req.into_body().into();

    let request = HttpRequest {
        body: Some(body.as_slice()),
        headers: &headers_slice,
        method,
        params: &[],
        uri: &uri,
    };

    host_send_http_request(request).map(|http_req| http_req.into())
}
