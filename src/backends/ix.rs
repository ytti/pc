use std::fmt::{self, Display, Formatter};

use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{override_if_present, override_option_with_option_none, serde_url};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(with = "serde_url")]
    pub url: Url,
    pub syntax: Option<String>,
    // pub reads: Option<u32>,
    pub username: Option<String>,
    pub apikey: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "ix backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Overrides url set in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,

    /// Filetype for syntax highlighting
    #[structopt(short = "s", long = "syntax", value_name = "filetype|NONE")]
    syntax: Option<String>,

    // /// Number of reads before paste is deleted
    // #[structopt(short = "R", long = "reads", value_name = "n reads|NONE")]
    // reads: Option<String>,
    /// Username to authenticate uploads (required if apikey set)
    #[structopt(short = "U", long = "username", value_name = "username|NONE")]
    pub username: Option<String>,

    /// API key to authenticate uploads (required if username set)
    #[structopt(short = "k", long = "apikey", value_name = "apikey|NONE")]
    pub apikey: Option<String>,
}

pub const NAME: &str = "ix";

pub const INFO: &str = r#"ix backend.
Supports ix.io.

Example config block:

    [servers.ix]
    backend = "ix"
    url = "http://ix.io/"

    # Optional values

    # Filetype for syntax highlighting.
    syntax = "python"

    # DISABLED - does not appear to work.
    # # Set number of reads before the paste is deleted. Default is unlimited reads.
    # reads = 2

    # Username and password for authenticated pastes; if these are both provided,
    # the program will attempt to authenticate with them. From ix.io: "If the login
    # does not exist, it will be created." Note that either both must be provided,
    # or neither. It is ok to provide them in stages - eg.  username in the config
    # file and api_token on the command line.
    username = "me"
    apikey = "hunter2"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.syntax, opt.syntax);
        override_option_with_option_none(&mut self.username, opt.username);
        override_option_with_option_none(&mut self.apikey, opt.apikey);
        // if let Some(new) = opt.reads {
        //     if new == "NONE" {
        //         self.reads = None;
        //     } else {
        //         self.reads = Some(new.parse().map_err(|x| clap::Error {
        //             message: format!("Invalid integer for --reads: {}", x),
        //             kind: clap::ErrorKind::InvalidValue,
        //             info: None,
        //         })?);
        //     }
        // }

        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("f:1", data);

        // let form = match self.reads {
        //     Some(reads) => form.text("read:1", reads.to_string()),
        //     None => form,
        // };

        let request = Client::new().post(self.url.clone()).multipart(form);

        let request = match (&self.username, &self.apikey) {
            (None, None) => request,
            (Some(ref username), Some(ref apikey)) => request.basic_auth(username, Some(apikey)),
            (_, _) => {
                return Err(
                    "Either both username and apikey must be provided, or neither."
                        .to_owned()
                        .into(),
                );
            }
        };

        let text = request.send()?.error_for_status()?.text()?;

        // check initial url for errors before adding any extra params
        if let Err(e) = Url::parse(&text) {
            return Err(format!(
                "could not parse response as url: {}\napi response body: {}",
                e, text
            )
            .into());
        }

        let text = match self.syntax {
            Some(ref syntax) => format!("{}/{}", text, syntax),
            None => text,
        };

        // now check again after adding the syntax param - user could have provided a value that
        // produces an invalid url.
        match Url::parse(&text) {
            Err(e) => Err(format!(
                "could not parse response as url after adding syntax url param: {}\nurl: {}",
                e, text
            )
            .into()),
            Ok(url) => Ok(url),
        }
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ix | {}", self.url)
    }
}
