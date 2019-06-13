use std::fmt::{self, Display, Formatter};

use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{override_if_present, override_option_with_option_none, serde_url};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UbuntuExpires {
    Day,
    Week,
    Month,
    Year,
}

impl Display for UbuntuExpires {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UbuntuExpires::Day => "day",
                UbuntuExpires::Week => "week",
                UbuntuExpires::Month => "month",
                UbuntuExpires::Year => "year",
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(with = "serde_url")]
    pub url: Url,
    pub syntax: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub expires: Option<UbuntuExpires>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "ubuntu backend")]
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
    /// Time to live as a duration
    #[structopt(short = "e", long = "expires", value_name = "day|week|month|year|NONE")]
    expires: Option<String>,
}

pub const NAME: &str = "ubuntu";

pub const INFO: &str = r#"Ubuntu paste backend.
Supports <https://paste.ubuntu.com/>.

Example config block:

    [servers.ubuntu]
    backend = "ubuntu"
    url = "https://paste.ubuntu.com/"

    # Optional values

    # Filetype for syntax highlighting.
    syntax = "js"

    # Approximate time to live. Default is use server default (no expiration, but not guaranteed).
    # Supported values are day, week, month, and year.
    expires = "week"

    # Username to publish as. Default is anonymous author.
    author = "my name"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.syntax, opt.syntax);
        override_option_with_option_none(&mut self.author, opt.author);
        if let Some(ref expires) = opt.expires {
            match expires.as_str() {
                "NONE" => {
                    self.expires = None;
                }
                "day" => {
                    self.expires = Some(UbuntuExpires::Day);
                }
                "week" => {
                    self.expires = Some(UbuntuExpires::Week);
                }
                "month" => {
                    self.expires = Some(UbuntuExpires::Month);
                }
                "year" => {
                    self.expires = Some(UbuntuExpires::Year);
                }
                e => {
                    return Err(clap::Error {
                        message: format!("Invalid value for expires: {}", e),
                        kind: clap::ErrorKind::InvalidValue,
                        info: None,
                    });
                }
            }
        }
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("content", data);
        let form = match self.syntax {
            None => form.text("syntax", "text".to_owned()),
            Some(ref syntax) => form.text("syntax", syntax.to_owned()),
        };
        let form = match self.author {
            Some(ref author) => form.text("poster", author.to_owned()),
            None => form,
        };
        let form = match self.expires {
            Some(expires) => form.text("expiration", expires.to_string()),
            None => form,
        };

        let res = Client::new()
            .post(self.url.clone())
            .multipart(form)
            .send()?;

        // Fails silently in some cases, for example if the syntax wasn't recognized. In this case
        // it redirects back to the main url. Web users will see a helpful message in the form.
        if res.url() == &self.url {
            Err("Paste failed.\nCheck parameters, it is possible that the syntax name provided wasn't recognized.".to_owned().into())
        } else {
            Ok(res.url().to_owned())
        }
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ubuntu | {}", self.url)
    }
}
