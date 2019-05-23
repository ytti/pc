mod generic;
mod haste;

pub use self::generic::GenericBackend;
pub use self::haste::HasteBackend;

pub fn backend_names() -> Vec<&'static str> {
    // NOTE: this should be manually kept up to date with enum variant names above
    vec!["generic", "haste"]
}

pub fn info_from_str(name: &str) -> Result<&'static str, String> {
    match name {
        "generic" => Ok(GenericBackend::info()),
        "haste" => Ok(HasteBackend::info()),
        s => Err(format!("{} is not a valid backend", s)),
    }
}
