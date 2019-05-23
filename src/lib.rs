use reqwest::Url;

mod backends;
mod error;
mod types;

use crate::backends::{GenericBackend, HastebinBackend};
use crate::error::PasteResult;
pub use crate::types::{BackendConfig, PasteClient};

/// Gives you a paste client implementation given config
pub fn build_client(config: &BackendConfig) -> PasteResult<Box<dyn PasteClient>> {
    match config {
        BackendConfig::Generic { url } => Ok(Box::new(GenericBackend::new(Url::parse(url)?))),
        BackendConfig::Hastebin { url } => Ok(Box::new(HastebinBackend::new(Url::parse(url)?))),
    }
}
