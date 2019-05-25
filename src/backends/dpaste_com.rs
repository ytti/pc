use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{deserialize_url, serialize_url};

// TODO: add support for syntax, expiry_days, poster
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    pub url: Url,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "dpaste_com backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
}

pub const NAME: &'static str = "dpaste_com";

pub fn info() -> &'static str {
    r#"Dpaste.com backend. Supports <http://dpaste.com/>.

Example config block:

    [servers.dpastecom]
    backend = "dpaste_com"
    url = "http://dpaste.com/""#
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
        // http://dpaste.com/api/v2/
        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/v2/");

        let form = Form::new().text("content", data);
        let text = Client::new()
            .post(api_endpoint)
            .multipart(form)
            .send()?
            .text()?;

        let url = Url::parse(&text)?;
        Ok(url)
    }
}
