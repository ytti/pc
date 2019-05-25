pub mod backends;
mod error;
mod types;
mod utils;

pub use crate::backends::BackendConfig;
pub use crate::backends::{fiche, generic, haste, modern_paste, vpaste};
pub use crate::types::PasteClient;

/// Provides a paste client implementation given config
pub fn build_client(config: BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic(backend) => Box::new(backend),
        BackendConfig::Haste(backend) => Box::new(backend),
        BackendConfig::Vpaste(backend) => Box::new(backend),
        BackendConfig::Fiche(backend) => Box::new(backend),
        BackendConfig::ModernPaste(backend) => Box::new(backend),
    }
}
