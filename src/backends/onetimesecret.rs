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
    pub passphrase: Option<String>,
    #[serde(default)]
    pub ttl: u64,
    pub recipient: Option<String>,
    pub username: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "onetimesecret backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
struct Opt {
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// password protect - default is no password ("NONE" = disable password)
    #[structopt(short = "p", long = "passphrase")]
    pub passphrase: Option<String>,
    /// time to live in seconds - default is set by the server ("0" = force use server default)
    #[structopt(short = "t", long = "ttl")]
    pub ttl: Option<u64>,
    /// email that the server should notify with the link; default is no email ("NONE" = disable this)
    #[structopt(short = "r", long = "recipient")]
    pub recipient: Option<String>,
    /// username for authenticated uploads ("NONE" = disable this)
    #[structopt(short = "n", long = "username")]
    pub username: Option<String>,
    /// api key for authenticated uploads ("NONE" = disable this)
    #[structopt(short = "k", long = "api-key")]
    pub api_key: Option<String>,
}

pub const NAME: &'static str = "onetimesecret";

pub const INFO: &'static str =
    r#"Paste backend to send text to onetimesecret servers <https://github.com/onetimesecret/onetimesecret>.

Example config block:

    [servers.ots]
    backend = "onetimesecret"
    url = "https://onetimesecret.com/"
    passphrase = "password123"
    ttl = 86400 # 1 day of seconds
    recipient = "user@example.com"
    username = "myuser@example.com"
    api_key = "DEADBEEF"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_if_present(&mut self.ttl, opt.ttl);
        override_option_with_option_none(&mut self.passphrase, opt.passphrase);
        override_option_with_option_none(&mut self.recipient, opt.recipient);
        override_option_with_option_none(&mut self.username, opt.username);
        override_option_with_option_none(&mut self.api_key, opt.api_key);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("secret", data);
        let form = match self.passphrase {
            Some(ref passphrase) => form.text("passphrase", passphrase.to_owned()),
            None => form,
        };
        let form = match self.ttl {
            0 => form,
            ttl => form.text("ttl", ttl.to_string()),
        };
        let form = match self.recipient {
            None => form,
            Some(ref r) => form.text("recipient", r.to_string()),
        };

        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/v1/share");

        let request = Client::new().post(api_endpoint).multipart(form);

        let request = match (&self.username, &self.api_key) {
            (None, None) => request,
            (Some(ref username), Some(ref api_key)) => request.basic_auth(username, Some(api_key)),
            (_, _) => {
                return Err(
                    "Either both username and api_key must be provided, or neither."
                        .to_owned()
                        .into(),
                );
            }
        };

        let data: String = request.send()?.text()?;

        let data: PasteResponse = match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(_) => {
                return Err(format!("api response: {}", data).into());
            }
        };

        let mut url: Url = self.url.clone();
        url.set_path(&format!("/secret/{}", data.secret_key));
        Ok(url)
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "onetimesecret | {}", self.url)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct PasteResponse {
    ttl: u64,
    secret_ttl: u64,
    passphrase_required: bool,
    custid: Option<String>,
    secret_key: String,
    metadata_key: String,
    metadata_ttl: u64,
    recipient: Vec<String>,
    created: u64,
    updated: u64,
}
