use reqwest::Client;
use serde::Deserialize;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;

/// Hastebin backend. Supports any servers running Haste
/// <https://github.com/seejohnrun/haste-server>. Official publicly available server for this is
/// <https://hastebin.com/>.
pub struct Haste {
    url: Url,
}

impl Haste {
    pub const NAME: &'static str = "haste";

    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn info() -> &'static str {
        r#"Hastebin backend. Supports any servers running Haste
<https://github.com/seejohnrun/haste-server>. Official publicly available server for this is
<https://hastebin.com/>.

Example config block:

    [servers.hastebin]
    backend = "haste"
    url = "https://hastebin.com/""#
    }
}

impl PasteClient for Haste {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let mut base_url = self.url.clone();

        base_url.set_path("documents");
        let info: HastePasteResponse = client.post(base_url.clone()).body(data).send()?.json()?;

        base_url.set_path(&info.key);
        Ok(base_url)
    }
}

#[derive(Deserialize)]
struct HastePasteResponse {
    key: String,
}
