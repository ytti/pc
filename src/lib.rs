pub mod backends;
mod error;
mod types;
mod utils;

pub use crate::backends::BackendConfig;
pub use crate::backends::{fiche, generic, haste, vpaste};
pub use crate::types::PasteClient;

/// Provides a paste client implementation given config
pub fn build_client(config: BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic(config) => Box::new(generic::new(config)),
        BackendConfig::Haste(config) => Box::new(haste::new(config)),
        BackendConfig::Vpaste(config) => Box::new(vpaste::new(config)),
        BackendConfig::Fiche(config) => Box::new(fiche::new(config)),
    }
}
