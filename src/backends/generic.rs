use std::fmt::{self, Display, Formatter};

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
#[structopt(about = "generic backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
struct Opt {
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
}

pub const NAME: &'static str = "generic";

pub const INFO: &'static str =
    r#"Generic paste service backend. Supports any pastebin services with the following two
properties:

1. data is uploaded via plain text in the POST request body to the base url.
2. the generated paste url is returned in plain text as the response body.

Example config block:

    [servers.paste_rs]
    backend = "generic"
    url = "https://paste.rs/""#;

impl PasteClient for Backend {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();
        let text = client.post(self.url.clone()).body(data).send()?.text()?;
        let url = Url::parse(&text)?;
        Ok(url)
    }

    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        if let Some(url) = opt.url {
            self.url = url;
        }
        Ok(())
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "generic | {}", self.url)
    }
}
