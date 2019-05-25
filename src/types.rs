use url::Url;

use crate::error::PasteResult;

pub trait PasteClient {
    fn paste(&self, data: String) -> PasteResult<Url>;
}
