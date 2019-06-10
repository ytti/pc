use std::fmt::{self, Display, Formatter};

use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{
    deserialize_url, override_if_present, override_option_with_option_none, serialize_url,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    pub url: Url,
    pub syntax: Option<String>,
    pub poster: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub expiry_days: u16,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "dpaste_com backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Override url in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// Set to NONE to use default, overriding any set in config file
    #[structopt(short = "s", long = "syntax")]
    syntax: Option<String>,
    /// Set to NONE to use default, overriding any set in config file
    #[structopt(short = "p", long = "poster")]
    poster: Option<String>,
    /// Set to NONE to use default, overriding any set in config file
    #[structopt(short = "t", long = "title")]
    title: Option<String>,
    /// Set to 0 to disable expiry
    #[structopt(short = "e", long = "expiry-days")]
    expiry_days: Option<u16>,
}

pub const NAME: &'static str = "dpaste_com";

pub const INFO: &'static str = r#"Dpaste.com backend. Supports <http://dpaste.com/>.

Example config block:

    [servers.dpastecom]
    backend = "dpaste_com"
    url = "http://dpaste.com/"

    # optional; default is plain text
    # see <http://dpaste.com/api/v2/syntax-choices/> for list of supported names
    syntax = "js"

    # optional; default is 0 (use server default (7 days)). If not 0, must be in inclusive range 1 - 365.
    expiry_days = 1

    # optional username to publish as; default is anonymous poster
    poster = "my name"

    # optional; default is no title
    title = "my paste"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.syntax, opt.syntax);
        override_option_with_option_none(&mut self.poster, opt.poster);
        override_option_with_option_none(&mut self.title, opt.title);
        if let Some(expiry_days) = opt.expiry_days {
            self.expiry_days = expiry_days;
        }
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        // http://dpaste.com/api/v2/
        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/v2/");

        let form = Form::new().text("content", data);
        let form = match self.syntax {
            Some(ref syntax) => form.text("syntax", syntax.to_owned()),
            None => form,
        };
        let form = match self.title {
            Some(ref title) => form.text("title", title.to_owned()),
            None => form,
        };
        let form = match self.poster {
            Some(ref poster) => form.text("poster", poster.to_owned()),
            None => form,
        };
        let form = match self.expiry_days {
            0 => form,
            e => form.text("expiry_days", e.to_string()),
        };

        let text = Client::new()
            .post(api_endpoint)
            .multipart(form)
            .send()?
            .text()?;

        match Url::parse(&text) {
            Err(e) => Err(format!(
                "could not parse response as url: {}\napi response body: {}",
                e, text
            )
            .into()),
            Ok(url) => Ok(url),
        }
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "dpaste_com | {}", self.url)
    }
}
