
mod backends;
mod error;
mod types;

use crate::backends::{GenericBackend, HastebinBackend};
use crate::error::PasteResult;
pub use crate::types::{BackendConfig, PasteClient};

/// Gives you a paste client implementation given config
pub fn build_client(config: BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic { url } => Box::new(GenericBackend::new(url)),
        BackendConfig::Hastebin { url } => Box::new(HastebinBackend::new(url)),
    }
}
