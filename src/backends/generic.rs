use reqwest::{Client, Url};

use crate::error::PasteResult;
use crate::types::PasteClient;

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
    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();
        let text = client.post(&self.url).body(data).send()?.text()?;
        let url = Url::parse(&text)?;
        Ok(url)
    }
}
