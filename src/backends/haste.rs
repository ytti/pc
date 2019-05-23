//! Hastebin backend. Supports any servers running Haste.
//!
//! Haste source: <https://github.com/seejohnrun/haste-server>.
//! Official publicly available server for this is <https://hastebin.com/>.
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{deserialize_url, serialize_url};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    pub url: Url,
}

#[derive(Debug, Clone)]
pub struct Backend {
    url: Url,
}

pub const NAME: &'static str = "haste";

pub fn new(config: Config) -> Backend {
    Backend { url: config.url }
}

pub fn info() -> &'static str {
    r#"Hastebin backend. Supports any servers running Haste
<https://github.com/seejohnrun/haste-server>. Official publicly available server for this is
<https://hastebin.com/>.

Example config block:

    [servers.hastebin]
    backend = "haste"
    url = "https://hastebin.com/""#
}

impl PasteClient for Backend {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let mut base_url = self.url.clone();

        base_url.set_path("documents");
        let info: HastePasteResponse = client.post(base_url.clone()).body(data).send()?.json()?;

        base_url.set_path(&info.key);
        Ok(base_url)
    }

    fn info(&self) -> &'static str {
        info()
    }

    fn name(&self) -> &'static str {
        NAME
    }
}

#[derive(Deserialize)]
struct HastePasteResponse {
    key: String,
}
