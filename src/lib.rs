mod backends;
mod error;
mod types;

use crate::backends::{GenericBackend, HastebinBackend};
pub use crate::types::{BackendConfig, PasteClient};

/// Gives you a paste client implementation given config
pub fn build_client(config: &BackendConfig) -> Box<dyn PasteClient> {
    match config {
        BackendConfig::Generic { url } => Box::new(GenericBackend::new(url.to_owned())),
        BackendConfig::Hastebin { url } => Box::new(HastebinBackend::new(url.to_owned())),
    }
}
