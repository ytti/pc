use reqwest::Client;
use reqwest::multipart::Form;
use serde::Serialize;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;

pub struct VpasteBackend {
    url: Url,
}

/// Vpaste backend. Supports any servers running Vpaste <http://vpaste.net/>.
///
/// Example config block:
///
///     [servers.vp]
///     backend = "vpaste"
///     url = "http://vpaste.net/"
impl VpasteBackend {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn info() -> &'static str {
        r#"Vpaste backend. Supports any servers running Vpaste <http://vpaste.net/>.

Example config block:

    [servers.vp]
    backend = "vpaste"
    url = "http://vpaste.net/""#
    }
}

impl PasteClient for VpasteBackend {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("text", data);
        let res = Client::new().post(self.url.clone()).multipart(form).send()?;
        Ok(res.url().to_owned())
    }
}


#[derive(Debug, Serialize)]
struct VpasteFormParams {
    text: String,
}
