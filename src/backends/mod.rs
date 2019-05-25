use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

pub mod fiche;
pub mod generic;
pub mod haste;
pub mod modern_paste;
pub mod vpaste;

pub const BACKEND_NAMES: &'static [&'static str] = &[
    generic::NAME,
    haste::NAME,
    vpaste::NAME,
    fiche::NAME,
    modern_paste::NAME,
];

pub fn info_from_str(name: &str) -> Result<&'static str, String> {
    match name {
        generic::NAME => Ok(generic::info()),
        haste::NAME => Ok(haste::info()),
        vpaste::NAME => Ok(vpaste::info()),
        fiche::NAME => Ok(fiche::info()),
        modern_paste::NAME => Ok(modern_paste::info()),
        s => Err(format!("{} is not a valid backend", s)),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    Generic(generic::Backend),
    Haste(haste::Backend),
    Vpaste(vpaste::Backend),
    Fiche(fiche::Backend),
    ModernPaste(modern_paste::Backend),
}

impl Display for BackendConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BackendConfig::Generic(generic::Backend { url }) => write!(f, "generic | {}", url),
            BackendConfig::Haste(haste::Backend { url }) => write!(f, "haste | {}", url),
            BackendConfig::Vpaste(vpaste::Backend { url }) => write!(f, "vpaste | {}", url),
            BackendConfig::Fiche(fiche::Backend { domain, port }) => {
                write!(f, "fiche | {}:{}", domain, port)
            }
            BackendConfig::ModernPaste(modern_paste::Backend { url }) => {
                write!(f, "modern_paste | {}", url)
            }
        }
    }
}
