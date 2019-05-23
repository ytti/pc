pub mod backends;
mod error;
mod types;
mod utils;

pub use crate::backends::BackendConfig;
use crate::backends::{generic, Fiche, Haste, Vpaste};
pub use crate::types::PasteClient;

/// Provides a paste client implementation given config
pub fn build_client(config: BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic(config) => Box::new(generic::new(config)),
        BackendConfig::Haste { url } => Box::new(Haste::new(url)),
        BackendConfig::Vpaste { url } => Box::new(Vpaste::new(url)),
        BackendConfig::Fiche { domain, port } => Box::new(Fiche::new(domain, port)),
    }
}
