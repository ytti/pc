mod generic;
mod haste;
mod vpaste;

pub use self::generic::Generic;
pub use self::haste::Haste;
pub use self::vpaste::Vpaste;

pub fn backend_names() -> Vec<&'static str> {
    // NOTE: this should be manually kept up to date with enum variant names above
    vec!["generic", "haste", "vpaste"]
}

pub fn info_from_str(name: &str) -> Result<&'static str, String> {
    match name {
        "generic" => Ok(Generic::info()),
        "haste" => Ok(Haste::info()),
        "vpaste" => Ok(Vpaste::info()),
        s => Err(format!("{} is not a valid backend", s)),
    }
}
