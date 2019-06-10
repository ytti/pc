use std::fmt::{self, Display, Formatter};

use reqwest::multipart::Form;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{deserialize_url, serialize_url, override_if_present};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(deserialize_with = "deserialize_url")]
    #[serde(serialize_with = "serialize_url")]
    pub url: Url,
    // TODO: other params: https://onetimesecret.com/docs/api/secrets
}

#[derive(Debug, StructOpt)]
#[structopt(about = "onetimesecret backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
struct Opt {
    /// Url
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
}

pub const NAME: &'static str = "onetimesecret";

pub const INFO: &'static str =
    r#"Paste backend to send text to onetimesecret servers <https://github.com/onetimesecret/onetimesecret>.

Example config block:

    [servers.ots]
    backend = "onetimesecret"
    url = "https://onetimesecret.com/""#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("secret", data);

        let mut api_endpoint: Url = self.url.clone();
        api_endpoint.set_path("/api/v1/share");

        let data: PasteResponse = Client::new()
            .post(api_endpoint)
            .multipart(form)
            .send()?
            .json()?;

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
    // NOTE: should add rest of response params in future.
    // https://onetimesecret.com/docs/api/secrets
    secret_key: String,
}
