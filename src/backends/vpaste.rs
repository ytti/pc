//! Vpaste backend. Supports any servers running Vpaste <http://vpaste.net/>.
//!
//! Example config block:
//!
//!     [servers.vp]
//!     backend = "vpaste"
//!     url = "http://vpaste.net/"
use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{deserialize_url, serialize_url};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    pub url: Url,
}

pub const NAME: &'static str = "vpaste";

pub fn info() -> &'static str {
    r#"Vpaste backend. Supports any servers running Vpaste <http://vpaste.net/>.

Example config block:

    [servers.vp]
    backend = "vpaste"
    url = "http://vpaste.net/""#
}

impl PasteClient for Backend {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("text", data);
        let res = Client::new()
            .post(self.url.clone())
            .multipart(form)
            .send()?;
        Ok(res.url().to_owned())
    }

    fn info(&self) -> &'static str {
        info()
    }

    fn name(&self) -> &'static str {
        NAME
    }
}
