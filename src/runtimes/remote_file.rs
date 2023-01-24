use serde::Deserialize;

/// A file represents a combination of both a remote URL and
/// a checksum. It will provide utils to download and validate
/// the file.
///
/// If the checksum is not present, the validation will return
/// always an Ok result.
///
#[derive(Deserialize)]
pub struct RemoteFile<'a> {
    /// URL pointing to the file
    pub url: &'a str,
    /// Checksum to validate the given file
    pub checksum: Checksum<'a>,
}

/// A list of available checksums. For now,
/// we will support only sha256
#[derive(Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Checksum<'a> {
    Sha256 { value: &'a str },
}
