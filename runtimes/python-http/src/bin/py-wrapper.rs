use pyo3::append_to_inittab;
use wlr_libpy::py_main::py_main;
use wws_http::py_module::wws_http;

pub fn main() {
    append_to_inittab!(wws_http);

    py_main(vec![String::from("--"), String::from("/app/backend.py")]);
}
