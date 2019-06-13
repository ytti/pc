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
    /// Overrides url set in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
    /// Password protects the secret
    #[structopt(short = "P", long = "password", value_name = "password|NONE")]
    pub password: Option<String>,
    /// Time to live as a duration
    #[structopt(short = "e", long = "expires", value_name = "duration|NONE")]
    pub expires: Option<String>,
    /// Instruct server to email recipient with link (only valid if authenticated)
    #[structopt(short = "r", long = "recipient", value_name = "email|NONE")]
    pub recipient: Option<String>,
    /// Username to authenticate uploads (required if apikey set)
    #[structopt(short = "U", long = "username", value_name = "username|NONE")]
    pub username: Option<String>,
    /// API key to authenticate uploads (required if username set)
    #[structopt(short = "k", long = "apikey", value_name = "apikey|NONE")]
    pub apikey: Option<String>,
}

pub const NAME: &str = "onetimesecret";

pub const INFO: &str = r#"Onetimesecret backend. 
Supports servers running <https://github.com/onetimesecret/onetimesecret>.

Example config block:

    [servers.ots]
    backend = "onetimesecret"
    url = "https://onetimesecret.com/"

    # Optional values

    # Password protect the secret.
    password = "password123"

    # Time to live as a duration.
    expires = "1day"

    # Instruct the server to email resulting link to this address (only works when authenticated).
    recipient = "user@example.com"

    # Authentication details. Either both must be set, or neither.
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
