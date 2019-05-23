use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

pub mod backends;
mod error;
mod types;

use crate::backends::{Fiche, Generic, Haste, Vpaste};
pub use crate::types::PasteClient;

/// Provides a paste client implementation given config
pub fn build_client(config: BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic { url } => Box::new(Generic::new(url)),
        BackendConfig::Haste { url } => Box::new(Haste::new(url)),
        BackendConfig::Vpaste { url } => Box::new(Vpaste::new(url)),
        BackendConfig::Fiche { domain, port } => Box::new(Fiche::new(domain, port)),
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

fn deserialize_url<'de, D>(d: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;

    match Url::parse(&s) {
        Ok(u) => Ok(u),
        Err(_) => Err(serde::de::Error::custom(format!(
            "Could not parse {:?} as a url",
            s
        ))),
    }
}

fn serialize_url<S>(x: &Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(x.as_str())
}
