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
    #[serde(default)]
    #[serde(with = "serde_humantime")]
    pub expires: Option<Duration>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "dpaste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Overrides url set in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// Filetype for syntax highlighting
    #[structopt(short = "s", long = "syntax", value_name = "filetype|NONE")]
    syntax: Option<String>,
    /// Time to live as a duration; see server config for extra supported values (eg. onetime, never)
    #[structopt(short = "e", long = "expires", value_name = "duration|NONE")]
    expires: Option<String>,
}

pub const NAME: &str = "dpaste";

pub const INFO: &str =
    r#"Dpaste backend. Supports any server running <https://github.com/bartTC/dpaste>.

Example config block:

    [servers.dpaste]
    backend = "dpaste"
    url = "https://dpaste.de/"

    # Optional values

    # Filetype for syntax highlighting. Default is set by the server.
    syntax = "python"

    # Paste lifetime as a duration. Default server config also supports special values like
    # "onetime" and "never".
    expires = "3600s"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.syntax, opt.syntax);
        override_option_duration_with_option_none(&mut self.expires, opt.expires)?;
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/");

        let form = Form::new().text("content", data).text("format", "url");
        let form = match self.syntax {
            Some(ref syntax) => form.text("lexer", syntax.to_owned()),
            None => form,
        };
        let form = match self.expires {
            Some(ref expires) => form.text("expires", expires.as_secs().to_string()),
            None => form,
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
        write!(f, "dpaste | {}", self.url)
    }
}
