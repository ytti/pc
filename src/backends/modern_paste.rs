use std::fmt::{self, Display, Formatter};
use std::time::{Duration, SystemTime};

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
    pub title: Option<String>,
    #[serde(default)]
    #[serde(with = "serde_humantime")]
    pub expires: Option<Duration>,
    pub syntax: Option<String>,
    pub password: Option<String>,
    pub apikey: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "modern_paste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Overrides url set in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// Title for the paste
    #[structopt(short = "t", long = "title", value_name = "title|NONE")]
    title: Option<String>,
    /// Time to live as a duration
    #[structopt(short = "e", long = "expires", value_name = "duration|NONE")]
    expires: Option<String>,
    /// Filetype for syntax highlighting
    #[structopt(short = "s", long = "syntax", value_name = "filetype|NONE")]
    syntax: Option<String>,
    /// Protects paste access with a password
    #[structopt(short = "P", long = "password", value_name = "password|NONE")]
    pub password: Option<String>,
    /// Upload paste as authenticated user
    #[structopt(short = "k", long = "apikey", value_name = "apikey|NONE")]
    pub apikey: Option<String>,
}

pub const NAME: &str = "modern_paste";

pub const INFO: &str = r#"Modern Paste backend.
Supports servers running <https://github.com/LINKIWI/modern-paste/>.
Example popular instance of this is <https://paste.fedoraproject.org/>.

Example config block:

    [servers.fedora]
    backend = "modern_paste"
    url = "https://paste.fedoraproject.org/"

    # Optional values

    title = "my paste" # default untitled
    expires = "3600s" # default is expiry set by server
    syntax = "python" # default plain text
    password = "password123" # default not password protected
    # default anonymous pastes
    apikey = "BbK1F09sZZXL2335iqDGvGeQswQUcvUmzxMoWjp3yvZDxpWwRiP4YQL6PiUA8gy2"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.title, opt.title);
        override_option_with_option_none(&mut self.password, opt.password);
        override_option_with_option_none(&mut self.apikey, opt.apikey);
        override_option_with_option_none(&mut self.syntax, opt.syntax);
        override_option_duration_with_option_none(&mut self.expires, opt.expires)?;
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let params = PasteParams {
            api_key: self.apikey.clone(),
            contents: data,
            expiry_time: match self.expires {
                None => None,
                Some(duration) => {
                    // api expects expiry as unix timestamp at which it expires
                    let expires = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("system time must be legit")
                        + duration;
                    Some(expires.as_secs())
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
