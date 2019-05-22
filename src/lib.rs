use serde::Deserialize;
use reqwest::Url;

mod error;

use crate::error::Result;

pub trait PasteClient {
    fn paste(&self, data: String) -> Result<String>;
    // TODO: help() function to return a help message as a string
}

#[derive(Debug, Deserialize)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    Generic { url: String },
    Hastebin { url: String },
}

pub fn build_client(config: &BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic { url } => Box::new(GenericBackend::new(url.to_owned())),
        BackendConfig::Hastebin { url } => Box::new(HastebinBackend::new(url.to_owned())),
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



/// Hastebin backend. Supports any servers running Haste
/// <https://github.com/seejohnrun/haste-server>. Official publicly available server for this is
/// <https://hastebin.com/>.
pub struct HastebinBackend {
    url: String,
}

impl HastebinBackend {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl PasteClient for HastebinBackend {
    fn paste(&self, data: String) -> Result<String> {
        let client = reqwest::Client::new();

        let mut base_url = Url::parse(&self.url)?;

        base_url.set_path("documents");
        let info: HastebinPasteResponse = client.post(base_url.clone()).body(data).send()?.json()?;

        base_url.set_path(&info.key);
        Ok(base_url.into_string())
    }
}

#[derive(Deserialize)]
struct HastebinPasteResponse {
    key: String,
}
