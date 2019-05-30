use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::types::PasteClient;

pub mod generic;
// pub mod haste;
// pub mod modern_paste;
// pub mod vpaste;
// pub mod dpaste_com;
// pub mod fiche;

lazy_static! {
        pub static ref BACKENDS_INFO: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(generic::NAME, generic::INFO);
        // TODO: others
        m
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    Generic(generic::Backend),
    // Haste(haste::Backend),
    // Vpaste(vpaste::Backend),
    // Fiche(fiche::Backend),
    // ModernPaste(modern_paste::Backend),
    // DpasteCom(dpaste_com::Backend),
}

impl BackendConfig {
    pub fn extract_backend(self) -> Box<dyn PasteClient> {
        match self {
            BackendConfig::Generic(backend) => Box::new(backend),
        }
    }
}
