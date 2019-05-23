use reqwest::{Client, Url};
use serde::Deserialize;

use crate::error::PasteResult;
use crate::types::PasteClient;

/// Hastebin backend. Supports any servers running Haste
/// <https://github.com/seejohnrun/haste-server>. Official publicly available server for this is
/// <https://hastebin.com/>.
pub struct HastebinBackend {
    url: Url,
}

impl HastebinBackend {
    pub fn new(url: Url) -> Self {
        Self { url }
    }
}

impl PasteClient for HastebinBackend {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let mut base_url = self.url.clone();

        base_url.set_path("documents");
        let info: HastebinPasteResponse =
            client.post(base_url.clone()).body(data).send()?.json()?;

        base_url.set_path(&info.key);
        Ok(base_url)
    }
}

#[derive(Deserialize)]
struct HastebinPasteResponse {
    key: String,
}
