use std::fmt::{self, Display, Formatter};
use std::time::SystemTime;

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
    pub title: Option<String>,
    pub expires: Option<String>,
    pub syntax: Option<String>,
    pub password: Option<String>,
    pub apikey: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "modern_paste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Title for the paste (NONE = untitled)
    #[structopt(short = "t", long = "title")]
    title: Option<String>,
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// time to live in seconds
    #[structopt(short = "e", long = "expires")]
    pub expires: Option<String>,
    /// language/filetype/syntax
    #[structopt(short = "s", long = "syntax")]
    pub syntax: Option<String>,
    /// protect paste access with this password (NONE = no password)
    #[structopt(short = "P", long = "password")]
    pub password: Option<String>,
    /// upload paste as authenticated user (NONE = force anon)
    #[structopt(short = "k", long = "apikey")]
    pub apikey: Option<String>,
}

pub const NAME: &str = "modern_paste";

pub const INFO: &str = r#"Modern Paste backend.
modern-paste source: <https://github.com/LINKIWI/modern-paste/>.
Example popular instance of this is <https://paste.fedoraproject.org/>.

Example config block:

    [servers.fedora]
    backend = "modern_paste"
    url = "https://paste.fedoraproject.org/"

    # optionals
    title = "my paste" # default untitled
    expires = "3600" # default is expiry set by server
    syntax = "python" # default plain text
    password = "password123" # default not password protected
    # default anonymous pastes
    apikey = "BbK1F09sZZXL2335iqDGvGeQswQUcvUmzxMoWjp3yvZDxpWwRiP4YQL6PiUA8gy2"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.expires, opt.expires);
        override_option_with_option_none(&mut self.title, opt.title);
        override_option_with_option_none(&mut self.password, opt.password);
        override_option_with_option_none(&mut self.apikey, opt.apikey);
        override_option_with_option_none(&mut self.syntax, opt.syntax);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let params = PasteParams {
            api_key: self.apikey.clone(),
            contents: data,
            expiry_time: match self.expires {
                None => None,
                Some(ref text) => {
                    // api expects expiry as unix timestamp at which it expires
                    let timestamp = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("system time must be legit")
                        .as_secs();
                    let expiry_time = timestamp + text.parse::<u64>()?;
                    Some(expiry_time)
                }
            },
            language: self.syntax.clone(),
            password: self.password.clone(),
            title: self.title.clone(),
        };

        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/paste/submit");
        let data: PasteResponse = client.post(api_endpoint).json(&params).send()?.json()?;

        if let Some(false) = data.success {
            return Err(format!("api returned failure: {:?}", data.message).into());
        }

        match data.url {
            None => Err("no url returned in response".to_owned().into()),
            Some(ref url) => {
                let url = Url::parse(url)?;
                Ok(url)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct PasteParams {
    contents: String,
    expiry_time: Option<u64>, // default: no expiry
    language: Option<String>, // default: plain text
    password: Option<String>, // default: no password
    title: Option<String>,    // default: "Untitled"
    api_key: Option<String>,  // default: anon
}

#[derive(Debug, Deserialize, Serialize)]
struct PasteResponse {
    // note: should add rest of response params in future.
    // https://paste.fedoraproject.org/api
    success: Option<bool>,
    message: Option<String>,
    failure_name: Option<String>,
    url: Option<String>,
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "modern_paste | {}", self.url)
    }
}
