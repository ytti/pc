use std::io::{Read, Write};
use std::net::TcpStream;

use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;

pub struct Fiche {
    domain: String,
    port: u16,
}

/// Fiche backend. Supports any servers running fiche <https://github.com/solusipse/fiche>. (Eg.
/// termbin.com)
///
/// Example config block:
///
///     [servers.termbin]
///     backend = "fiche"
///     url = "termbin.com"
///     # default port if missing is 9999
///     port = 9999
impl Fiche {
    pub fn new(domain: String, port: u16) -> Self {
        Self { domain, port }
    }

    pub fn default_port() -> u16 {
        9999
    }

    pub fn info() -> &'static str {
        r#"Fiche backend. Supports any servers running fiche <https://github.com/solusipse/fiche>. (Eg.
termbin.com)

Example config block:

    [servers.termbin]
    backend = "fiche"
    url = "termbin.com"
    # default port if missing is 9999
    port = 9999"#
    }
}

impl PasteClient for Fiche {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.domain, self.port))?;

        stream.write(data.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        let sanitized_data = response.trim_matches(char::from(0)).trim_end();
        let url = Url::parse(sanitized_data)?;
        Ok(url)
    }
}
