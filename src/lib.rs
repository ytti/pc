use serde::Deserialize;

mod error;

use crate::error::Result;

pub trait PasteClient {
    fn paste(&self, data: String) -> Result<String>;
}

#[derive(Debug, Deserialize)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
pub enum BackendConfig {
    #[serde(rename = "generic")]
    Generic { url: String },
}

pub fn build_client(config: &BackendConfig) -> impl PasteClient {
    match config {
        BackendConfig::Generic { url } => GenericBackend::new(url.to_owned()),
    }
}

/// Generic paste service backend. Supports any pastebin services with the following two
/// properties:
///
/// 1. data is uploaded via plain text in the POST request body to the base url.
/// 2. the generated paste url is returned in plain text as the response body.
pub struct GenericBackend {
    url: String,
}

impl GenericBackend {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl PasteClient for GenericBackend {
    fn paste(&self, data: String) -> Result<String> {
        let client = reqwest::Client::new();
        let text = client.post(&self.url).body(data).send()?.text()?;
        Ok(text)
    }
}
