//! Modern Paste backend.
//!
//! modern-paste source: <https://github.com/LINKIWI/modern-paste/>.
//! Example popular instance of this is <https://paste.fedoraproject.org/>.
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

pub const NAME: &'static str = "modern_paste";

pub fn new(config: Config) -> Backend {
    Backend { url: config.url }
}

pub fn info() -> &'static str {
    r#"Modern Paste backend.
modern-paste source: <https://github.com/LINKIWI/modern-paste/>.
Example popular instance of this is <https://paste.fedoraproject.org/>.

Example config block:

    [servers.fedora]
    backend = "modern_paste"
    url = "https://paste.fedoraproject.org/""#
}

impl PasteClient for Backend {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let params = PasteParams {
            contents: data,
            expiry_time: None,
            language: None,
            password: None,
            title: None,
        };

        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/paste/submit");
        let data: PasteResponse = client.post(api_endpoint).json(&params).send()?.json()?;

        match data.success {
            Some(true) => {
                return Err("api returned success: false".to_owned().into());
            }
            _ => {}
        }

        Ok(data.url)
    }

    fn info(&self) -> &'static str {
        info()
    }

    fn name(&self) -> &'static str {
        NAME
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct PasteParams {
    contents: String,
    expiry_time: Option<u32>, // default: no expiry
    language: Option<String>, // default: plain text
    password: Option<String>, // default: no password
    title: Option<String>,    // default: "Untitled"
}

#[derive(Debug, Deserialize, Serialize)]
struct PasteResponse {
    // note: should add rest of response params in future.
    // https://paste.fedoraproject.org/api
    success: Option<bool>,
    message: Option<String>,
    contents: String,
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    url: Url,
}
