use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
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

#[derive(Debug, StructOpt)]
#[structopt(about = "modern_paste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
}

pub const NAME: &'static str = "vpaste";

pub fn info() -> &'static str {
    r#"Vpaste backend. Supports any servers running Vpaste <http://vpaste.net/>.

Example config block:

    [servers.vp]
    backend = "vpaste"
    url = "http://vpaste.net/""#
}

impl Backend {
    pub fn apply_args(self, args: Vec<String>) -> clap::Result<Box<dyn PasteClient>> {
        let opt = Opt::from_iter_safe(args)?;
        Ok(Box::new(Self {
            url: opt.url.unwrap_or(self.url),
        }))
    }
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
}
