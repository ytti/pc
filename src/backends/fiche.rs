use std::fmt::{self, Display, Formatter};
use std::io::{Read, Write};
use std::net::TcpStream;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    pub domain: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "fiche backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// domain
    #[structopt(short = "d", long = "domain")]
    domain: Option<String>,

    /// port
    #[structopt(short = "p", long = "port")]
    port: Option<u16>,
}

pub const NAME: &'static str = "fiche";

pub fn default_port() -> u16 {
    9999
}

pub const INFO: &'static str =
    r#"Fiche backend. Supports any servers running fiche <https://github.com/solusipse/fiche>. (Eg.
termbin.com)

Example config block:

    [servers.termbin]
    backend = "fiche"
    url = "termbin.com"
    # default port if missing is 9999
    port = 9999"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        if let Some(domain) = opt.domain {
            self.domain = domain;
        }
        if let Some(port) = opt.port {
            self.port = port;
        }
        Ok(())
    }

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

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "fiche | {}:{}", self.domain, self.port)
    }
}
