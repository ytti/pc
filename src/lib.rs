
mod backends;
mod error;
mod types;

use crate::backends::{GenericBackend, HasteBackend};
pub use crate::types::{BackendConfig, PasteClient};

/// Gives you a paste client implementation given config
pub fn build_client(config: BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic { url } => Box::new(GenericBackend::new(url)),
        BackendConfig::Haste { url } => Box::new(HasteBackend::new(url)),
    }
}
