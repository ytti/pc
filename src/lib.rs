pub mod backends;
mod error;
mod types;
mod utils;

pub use crate::backends::BackendConfig;
pub use crate::backends::{fiche, generic, haste, modern_paste, vpaste};
pub use crate::types::PasteClient;

/// Provides a paste client implementation given config
pub fn build_client(config: BackendConfig, override_args: Vec<String>) -> clap::Result<Box<dyn PasteClient>> {
    match config {
        BackendConfig::Generic(backend) => backend.apply_args(override_args),
        BackendConfig::Haste(backend) => backend.apply_args(override_args),
        BackendConfig::Vpaste(backend) => backend.apply_args(override_args),
        BackendConfig::Fiche(backend) => backend.apply_args(override_args),
        BackendConfig::ModernPaste(backend) => backend.apply_args(override_args),
    }
}
