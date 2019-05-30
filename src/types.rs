use std::fmt::Display;

use url::Url;

use crate::error::PasteResult;

pub trait PasteClient: Display {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()>;
    fn paste(&self, data: String) -> PasteResult<Url>;
}
