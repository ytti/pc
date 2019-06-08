use std::fmt::{self, Display, Formatter};

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
    pub language: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "modern_paste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,

    /// Optional language for syntax highlighting (default plain text; NONE = force default)
    #[structopt(short = "l", long = "language")]
    language: Option<String>,
}

pub const NAME: &'static str = "sprunge";

pub const INFO: &'static str =
    r#"Sprunge backend. Supports any servers running sprunge <https://github.com/rupa/sprunge>.

Example config block:

    [servers.sprunge]
    backend = "sprunge"
    url = "http://sprunge.us/"
    language = "py"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        if let Some(url) = opt.url {
            self.url = url;
        }
        if let Some(language) = opt.language {
            if language == "NONE" {
                self.language = None;
            } else {
                self.language = Some(language);
            }
        }
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("sprunge", data);
        let text = Client::new()
            .post(self.url.clone())
            .multipart(form)
            .send()?
            .text()?;
        let mut url = Url::parse(&text)?;
        if let Some(ref lang) = self.language {
            url.set_query(Some(lang));
        }
        Ok(url)
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "sprunge | {}", self.url)
    }
}
