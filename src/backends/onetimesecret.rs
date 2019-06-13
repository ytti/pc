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
    pub password: Option<String>,
    #[serde(default)]
    #[serde(with = "serde_humantime")]
    pub expires: Option<Duration>,
    pub recipient: Option<String>,
    pub username: Option<String>,
    pub apikey: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "onetimesecret backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
struct Opt {
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// password protect - default is no password ("NONE" = disable password)
    #[structopt(short = "P", long = "password")]
    pub password: Option<String>,
    /// time to live in seconds - default is set by the server (NONE = force use server default)
    #[structopt(short = "e", long = "expires")]
    pub expires: Option<String>,
    /// email that the server should notify with the link; default is no email ("NONE" = disable this)
    #[structopt(short = "r", long = "recipient")]
    pub recipient: Option<String>,
    /// username for authenticated uploads ("NONE" = disable this)
    #[structopt(short = "U", long = "username")]
    pub username: Option<String>,
    /// api key for authenticated uploads ("NONE" = disable this)
    #[structopt(short = "k", long = "apikey")]
    pub apikey: Option<String>,
}

pub const NAME: &str = "onetimesecret";

pub const INFO: &str =
    r#"Paste backend to send text to onetimesecret servers <https://github.com/onetimesecret/onetimesecret>.

Example config block:

    [servers.ots]
    backend = "onetimesecret"
    url = "https://onetimesecret.com/"

    # optionals
    password = "password123"
    expires = "1day"
    recipient = "user@example.com"
    username = "myuser@example.com"
    apikey = "DEADBEEF"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.password, opt.password);
        override_option_with_option_none(&mut self.recipient, opt.recipient);
        override_option_with_option_none(&mut self.username, opt.username);
        override_option_with_option_none(&mut self.apikey, opt.apikey);
        override_option_duration_with_option_none(&mut self.expires, opt.expires)?;
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("secret", data);
        let form = match self.password {
            Some(ref password) => form.text("passphrase", password.to_owned()),
            None => form,
        };
        let form = match self.expires {
            None => form,
            Some(duration) => form.text("ttl", duration.as_secs().to_string()),
        };
        let form = match self.recipient {
            None => form,
            Some(ref r) => form.text("recipient", r.to_string()),
        };

        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/v1/share");

        let request = Client::new().post(api_endpoint).multipart(form);

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

        let data: String = request.send()?.text()?;

        let data: PasteResponse = match serde_json::from_str(&data) {
            Ok(data) => {
                dbg!(&data);
                data
            }
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
