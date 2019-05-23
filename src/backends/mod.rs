use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

pub mod fiche;
pub mod generic;
pub mod haste;
pub mod vpaste;

pub use self::fiche::Fiche;
pub use self::haste::Haste;
pub use self::vpaste::Vpaste;
use crate::utils::{deserialize_url, serialize_url};

pub const BACKEND_NAMES: &'static [&'static str] =
    &[generic::NAME, Haste::NAME, Vpaste::NAME, Fiche::NAME];

pub fn info_from_str(name: &str) -> Result<&'static str, String> {
    match name {
        generic::NAME => Ok(generic::info()),
        Haste::NAME => Ok(Haste::info()),
        Vpaste::NAME => Ok(Vpaste::info()),
        Fiche::NAME => Ok(Fiche::info()),
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
    Generic(generic::Config),
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
            BackendConfig::Generic(generic::Config { url }) => write!(f, "generic | {}", url),
            BackendConfig::Haste { url } => write!(f, "haste | {}", url),
            BackendConfig::Vpaste { url } => write!(f, "vpaste | {}", url),
            BackendConfig::Fiche { domain, port } => write!(f, "fiche | {}:{}", domain, port),
        }
    }
}
