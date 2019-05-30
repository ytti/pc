use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::types::PasteClient;

pub mod dpaste_com;
pub mod fiche;
pub mod generic;
pub mod haste;
pub mod modern_paste;
pub mod onetimesecret;
pub mod vpaste;

lazy_static! {
    pub static ref BACKENDS_INFO: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(dpaste_com::NAME, dpaste_com::INFO);
        m.insert(fiche::NAME, fiche::INFO);
        m.insert(generic::NAME, generic::INFO);
        m.insert(haste::NAME, haste::INFO);
        m.insert(modern_paste::NAME, modern_paste::INFO);
        m.insert(onetimesecret::NAME, onetimesecret::INFO);
        m.insert(vpaste::NAME, vpaste::INFO);
        m
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    DpasteCom(dpaste_com::Backend),
    Fiche(fiche::Backend),
    Generic(generic::Backend),
    Haste(haste::Backend),
    ModernPaste(modern_paste::Backend),
    Onetimesecret(onetimesecret::Backend),
    Vpaste(vpaste::Backend),
}

impl BackendConfig {
    pub fn extract_backend(self) -> Box<dyn PasteClient> {
        match self {
            BackendConfig::DpasteCom(backend) => Box::new(backend),
            BackendConfig::Fiche(backend) => Box::new(backend),
            BackendConfig::Generic(backend) => Box::new(backend),
            BackendConfig::Haste(backend) => Box::new(backend),
            BackendConfig::ModernPaste(backend) => Box::new(backend),
            BackendConfig::Onetimesecret(backend) => Box::new(backend),
            BackendConfig::Vpaste(backend) => Box::new(backend),
        }
    }
}
