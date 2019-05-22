use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::error::PasteResult;

pub trait PasteClient {
    fn paste(&self, data: String) -> PasteResult<Url>;
    // TODO: help() function to return a help message as a string
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    Generic { url: String },
    Hastebin { url: String },
}
