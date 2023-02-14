use base64::{engine::general_purpose, Engine as _};

/// Identifies the content of a response. In other words, the body.
/// We need this intermediate entity in Rust to be able to expose
/// an array of bytes as response.
///
/// Note that Wasm Workers Server interacts with modules via
/// serialized UTF-8 JSONs. An array of bytes response may include
/// bytes that cannot be represented as UTF-8. To avoid this
/// limitation, Content is able to encode them as base64. Then,
/// wws will ensure to decode them before sending the bytes to the
/// client.
pub enum Content {
    Text(String),
    Base64(String),
}

impl From<Vec<u8>> for Content {
    fn from(s: Vec<u8>) -> Content {
        Content::Base64(general_purpose::STANDARD.encode(s))
    }
}

impl From<String> for Content {
    fn from(s: String) -> Content {
        Content::Text(s)
    }
}
