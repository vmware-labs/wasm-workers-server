use pyo3::prelude::*;
use pyo3::{types::PyModule, PyResult, Python};

wit_bindgen_rust::import!({paths: ["../../wit/core/http.wit"]});

use self::http::{
    send_http_request as wasi_send_http_request, HttpMethod, HttpRequest, HttpResponse,
};

pub use self::http::HttpRequestError;

#[pyo3::pyfunction]
pub fn send_http_request(uri: String) -> (String, bool) {
    let result = wasi_send_http_request(HttpRequest {
        body: Some(Default::default()),
        headers: Default::default(),
        method: HttpMethod::Get,
        params: Default::default(),
        uri: &uri,
    });

    match result {
        Ok(result) => (
            String::from_utf8_lossy(result.body.unwrap_or_default().as_ref()).into_owned(),
            true,
        ),
        Err(err) => (format!("{:?}", err).into(), false),
    }
}

#[pyo3::pymodule]
pub fn wws_http(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(send_http_request, module)?)?;

    Ok(())
}
