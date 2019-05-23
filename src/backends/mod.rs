use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

pub mod fiche;
pub mod generic;
pub mod haste;
pub mod vpaste;

pub const BACKEND_NAMES: &'static [&'static str] =
    &[generic::NAME, haste::NAME, vpaste::NAME, fiche::NAME];

pub fn info_from_str(name: &str) -> Result<&'static str, String> {
    match name {
        generic::NAME => Ok(generic::info()),
        haste::NAME => Ok(haste::info()),
        vpaste::NAME => Ok(vpaste::info()),
        fiche::NAME => Ok(fiche::info()),
        s => Err(format!("{} is not a valid backend", s)),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    Generic(generic::Config),
    Haste(haste::Config),
    Vpaste(vpaste::Config),
    Fiche(fiche::Config),
}

impl Display for BackendConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BackendConfig::Generic(generic::Config { url }) => write!(f, "generic | {}", url),
            BackendConfig::Haste(haste::Config { url }) => write!(f, "haste | {}", url),
            BackendConfig::Vpaste(vpaste::Config { url }) => write!(f, "vpaste | {}", url),
            BackendConfig::Fiche(fiche::Config { domain, port }) => {
                write!(f, "fiche | {}:{}", domain, port)
            }
        }
    }
}
