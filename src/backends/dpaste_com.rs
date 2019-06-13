use std::fmt::{self, Display, Formatter};
use std::time::Duration;

use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{
    override_if_present, override_option_duration_with_option_none,
    override_option_with_option_none, serde_humantime, serde_url,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(with = "serde_url")]
    pub url: Url,
    pub syntax: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    #[serde(with = "serde_humantime")]
    pub expires: Option<Duration>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "dpaste_com backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Overrides url set in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// Filetype for syntax highlighting
    #[structopt(short = "s", long = "syntax", value_name = "filetype|NONE")]
    syntax: Option<String>,
    /// Sets a name for the paste author
    #[structopt(short = "a", long = "author", value_name = "author|NONE")]
    author: Option<String>,
    /// Title for the paste
    #[structopt(short = "t", long = "title", value_name = "title|NONE")]
    title: Option<String>,
    /// Time to live as a duration
    #[structopt(short = "e", long = "expires", value_name = "duration|NONE")]
    expires: Option<String>,
}

pub const NAME: &str = "dpaste_com";

pub const INFO: &str = r#"Dpaste.com backend.
Supports <http://dpaste.com/>.

Example config block:

    [servers.dpastecom]
    backend = "dpaste_com"
    url = "http://dpaste.com/"

    # Optional values

    # Filetype for syntax highlighting. See <http://dpaste.com/api/v2/syntax-choices/> for list of
    # supported names.
    syntax = "js"

    # Time to live as a duration. Default is use server default (7 days).
    # Max duration supported by server is 1year.
    expires = "1d"

    # Username to publish as. Default is anonymous author.
    author = "my name"

    # Title for the paste.
    title = "my paste"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.syntax, opt.syntax);
        override_option_with_option_none(&mut self.author, opt.author);
        override_option_with_option_none(&mut self.title, opt.title);
        override_option_duration_with_option_none(&mut self.expires, opt.expires)?;
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
        let form = match self.author {
            Some(ref author) => form.text("poster", author.to_owned()),
            None => form,
        };
        let form = match self.expires {
            None => form,
            Some(ref duration) => {
                // the api expects an expiry in days
                let expiry_days: u64 = duration.as_secs() / 60 / 60 / 24;
                form.text("expiry_days", expiry_days.to_string())
            }
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
