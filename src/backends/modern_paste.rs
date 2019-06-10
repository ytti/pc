use std::fmt::{self, Display, Formatter};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{
    deserialize_url, override_if_present, override_option_if_present,
    override_option_with_option_none, serialize_url,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    pub url: Url,
    pub title: Option<String>,
    /// unix timestamp at which the paste should expire
    #[serde(default)]
    pub expiry_time: u64,
    pub language: Option<String>,
    pub password: Option<String>,
    pub api_key: Option<String>,
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
    /// unix timestamp at which the paste should expire (0 = use server default expiry)
    #[structopt(short = "e", long = "expiry-time")]
    pub expiry_time: Option<u64>,
    /// language/filetype/syntax
    #[structopt(short = "l", long = "language")]
    pub language: Option<String>,
    /// protect paste access with this password (NONE = no password)
    #[structopt(short = "p", long = "password")]
    pub password: Option<String>,
    /// upload paste as authenticated user (NONE = force anon)
    #[structopt(short = "k", long = "api-key")]
    pub api_key: Option<String>,
}

pub const NAME: &'static str = "modern_paste";

pub const INFO: &'static str = r#"Modern Paste backend.
modern-paste source: <https://github.com/LINKIWI/modern-paste/>.
Example popular instance of this is <https://paste.fedoraproject.org/>.

Example config block:

    [servers.fedora]
    backend = "modern_paste"
    url = "https://paste.fedoraproject.org/"

    # optionals
    title = "my paste" # default untitled
    expiry_time = 1559817269 # default is expiry set by server
    language = "python" # default plain text
    password = "password123" # default not password protected
    # default anonymous pastes
    api_key = "BbK1F09sZZXL2335iqDGvGeQswQUcvUmzxMoWjp3yvZDxpWwRiP4YQL6PiUA8gy2"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_if_present(&mut self.expiry_time, opt.expiry_time);
        override_option_with_option_none(&mut self.title, opt.title);
        override_option_with_option_none(&mut self.password, opt.password);
        override_option_with_option_none(&mut self.api_key, opt.api_key);
        override_option_if_present(&mut self.language, opt.language);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let params = PasteParams {
            api_key: self.api_key.clone(),
            contents: data,
            expiry_time: match self.expiry_time.clone() {
                0 => None,
                e => Some(e),
            },
            language: self.language.clone(),
            password: self.password.clone(),
            title: self.title.clone(),
        };

        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/paste/submit");
        let data: PasteResponse = client.post(api_endpoint).json(&params).send()?.json()?;

        if let Some(false) = data.success {
            return Err(format!("api returned failure: false {:?}", data.failure_name).into());
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
