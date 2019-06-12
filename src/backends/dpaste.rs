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
    pub lexer: Option<String>,
    pub expires: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "dpaste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Override url in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// syntax/filetype - set to NONE to use default, overriding any set in config file
    #[structopt(short = "l", long = "lexer")]
    lexer: Option<String>,
    /// lifetime of paste in seconds. See server config for extra supported values (eg. onetime,
    /// never). Set to NONE to disable.
    #[structopt(short = "e", long = "expires")]
    expires: Option<String>,
}

pub const NAME: &str = "dpaste";

pub const INFO: &str =
    r#"Dpaste backend. Supports any server running <https://github.com/bartTC/dpaste>.

Example config block:

    [servers.dpaste]
    backend = "dpaste"
    url = "https://dpaste.de/"

    # optional; syntax highlighting / filetype (default is set by server)
    lexer = "python"

    # optional; lifetime in seconds. Default server config also supports special values like
    # "onetime" and "never".
    expires = "2592000"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.lexer, opt.lexer);
        override_option_with_option_none(&mut self.expires, opt.expires);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        dbg!(&self);
        // http://dpaste.com/api/v2/
        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/");

        let form = Form::new().text("content", data).text("format", "url");
        let form = match self.lexer {
            Some(ref lexer) => form.text("lexer", lexer.to_owned()),
            None => form,
        };
        let form = match self.expires {
            Some(ref expires) => form.text("expires", expires.to_owned()),
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
