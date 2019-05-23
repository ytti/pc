mod generic;
mod haste;
mod vpaste;

pub use self::generic::GenericBackend;
pub use self::haste::HasteBackend;
pub use self::vpaste::VpasteBackend;

pub fn backend_names() -> Vec<&'static str> {
    // NOTE: this should be manually kept up to date with enum variant names above
    vec!["generic", "haste", "vpaste"]
}

pub fn info_from_str(name: &str) -> Result<&'static str, String> {
    match name {
        "generic" => Ok(GenericBackend::info()),
        "haste" => Ok(HasteBackend::info()),
        "vpaste" => Ok(VpasteBackend::info()),
        s => Err(format!("{} is not a valid backend", s)),
    }
}
