//! Fiche backend. Supports any servers running fiche <https://github.com/solusipse/fiche>. (Eg.
//! termbin.com)
//!
//! Example config block:
//!
//!     [servers.termbin]
//!     backend = "fiche"
//!     url = "termbin.com"
//!     # default port if missing is 9999
//!     port = 9999
use std::io::{Read, Write};
use std::net::TcpStream;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub domain: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Backend {
    domain: String,
    port: u16,
}

pub const NAME: &'static str = "fiche";

pub fn new(config: Config) -> Backend {
    Backend {
        domain: config.domain,
        port: config.port,
    }
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

impl PasteClient for Backend {
    fn paste(&self, data: String) -> PasteResult<Url> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.domain, self.port))?;

        stream.write(data.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        let sanitized_data = response.trim_matches(char::from(0)).trim_end();
        let url = Url::parse(sanitized_data)?;
        Ok(url)
    }

    fn info(&self) -> &'static str {
        info()
    }

    fn name(&self) -> &'static str {
        NAME
    }
}
