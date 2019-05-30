use std::fmt::{self, Display, Formatter};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{deserialize_url, serialize_url};

// TODO: add support for other optional config ( https://paste.fedoraproject.org/api )
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    pub url: Url,
    pub title: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "modern_paste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Title for the paste
    #[structopt(short = "t", long = "title")]
    title: Option<String>,

    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
}

pub const NAME: &'static str = "modern_paste";

pub const INFO: &'static str = r#"Modern Paste backend.
modern-paste source: <https://github.com/LINKIWI/modern-paste/>.
Example popular instance of this is <https://paste.fedoraproject.org/>.

Example config block:

    [servers.fedora]
    backend = "modern_paste"
    url = "https://paste.fedoraproject.org/""#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        if let Some(url) = opt.url {
            self.url = url;
        }
        if opt.title.is_some() {
            self.title = opt.title;
        }
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let params = PasteParams {
            contents: data,
            expiry_time: None,
            language: None,
            password: None,
            title: self.title.clone(),
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

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "modern_paste | {}", self.url)
    }
}
