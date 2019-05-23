use url::Url;

use crate::error::PasteResult;

pub trait PasteClient {
    fn paste(&self, data: String) -> PasteResult<Url>;
    fn info(&self) -> &'static str;
    fn name(&self) -> &'static str;
}
