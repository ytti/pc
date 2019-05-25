use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;
use structopt::StructOpt;

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
#[structopt(about = "haste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
}

pub const NAME: &'static str = "haste";

pub fn info() -> &'static str {
    r#"Hastebin backend. Supports any servers running Haste
<https://github.com/seejohnrun/haste-server>. Official publicly available server for this is
<https://hastebin.com/>.

Example config block:

    [servers.hastebin]
    backend = "haste"
    url = "https://hastebin.com/""#
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
