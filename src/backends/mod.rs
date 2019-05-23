use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

mod fiche;
mod generic;
mod haste;
mod vpaste;

pub use self::fiche::Fiche;
pub use self::generic::{Generic, GenericConfig};
pub use self::haste::Haste;
pub use self::vpaste::Vpaste;
use crate::utils::{deserialize_url, serialize_url};

pub fn backend_names() -> Vec<&'static str> {
    // NOTE: this should be manually kept up to date with enum variant names above
    vec!["generic", "haste", "vpaste", "fiche"]
}

pub fn info_from_str(name: &str) -> Result<&'static str, String> {
    match name {
        "generic" => Ok(Generic::info()),
        "haste" => Ok(Haste::info()),
        "vpaste" => Ok(Vpaste::info()),
        "fiche" => Ok(Fiche::info()),
        s => Err(format!("{} is not a valid backend", s)),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    // TODO: these structs should be self contained structs in each backend and enum values simply
    // wrap them? At the moment information about a backend is split between the backend
    // file/struct and the config.
    Generic {
        #[serde(deserialize_with = "deserialize_url")]
        #[serde(serialize_with = "serialize_url")]
        url: Url,
    },
    Haste {
        #[serde(deserialize_with = "deserialize_url")]
        #[serde(serialize_with = "serialize_url")]
        url: Url,
    },
    Vpaste {
        #[serde(deserialize_with = "deserialize_url")]
        #[serde(serialize_with = "serialize_url")]
        url: Url,
    },
    Fiche {
        domain: String,
        #[serde(default = "Fiche::default_port")]
        port: u16,
    },
}

impl Display for BackendConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BackendConfig::Generic { url } => write!(f, "generic | {}", url),
            BackendConfig::Haste { url } => write!(f, "haste | {}", url),
            BackendConfig::Vpaste { url } => write!(f, "vpaste | {}", url),
            BackendConfig::Fiche { domain, port } => write!(f, "fiche | {}:{}", domain, port),
        }
    }
}
