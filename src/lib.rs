pub mod backends;
mod error;
mod types;

use crate::backends::{Fiche, Generic, Haste, Vpaste};
pub use crate::types::{BackendConfig, PasteClient};

/// Provides a paste client implementation given config
pub fn build_client(config: BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic { url } => Box::new(Generic::new(url)),
        BackendConfig::Haste { url } => Box::new(Haste::new(url)),
        BackendConfig::Vpaste { url } => Box::new(Vpaste::new(url)),
        BackendConfig::Fiche { domain, port } => Box::new(Fiche::new(domain, port)),
    }
}
