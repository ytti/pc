use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;

use crate::utils::{deserialize_url, serialize_url};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct GenericConfig {
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    url: Url,
}

/// Generic paste service backend. Supports any pastebin services with the following two
/// properties:
///
/// 1. data is uploaded via plain text in the POST request body to the base url.
/// 2. the generated paste url is returned in plain text as the response body.
pub struct Generic {
    url: Url,
}

impl Generic {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn info() -> &'static str {
        r#"Generic paste service backend. Supports any pastebin services with the following two
properties:

1. data is uploaded via plain text in the POST request body to the base url.
2. the generated paste url is returned in plain text as the response body.

Example config block:

    [servers.paste_rs]
    backend = "generic"
    url = "https://paste.rs/""#
    }
}

impl PasteClient for Generic {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();
        let text = client.post(self.url.clone()).body(data).send()?.text()?;
        let url = Url::parse(&text)?;
        Ok(url)
    }
}
