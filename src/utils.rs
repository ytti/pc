use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use serde::Deserialize;
use serde::{Deserializer, Serializer};
use url::Url;

pub fn deserialize_url<'de, D>(d: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;

    match Url::parse(&s) {
        Ok(u) => Ok(u),
        Err(_) => Err(serde::de::Error::custom(format!(
            "Could not parse {:?} as a url",
            s
        ))),
    }
}

pub fn serialize_url<S>(x: &Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(x.as_str())
}

pub fn read_file(fname: &str) -> io::Result<String> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn write_hist(paste_url: Url, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    file.write_all(format!("{}\n", paste_url).as_bytes())?;
    Ok(())
}

/// when the current value is an optional string and needs to be optionally overridden with a
/// string, or forced to None with an explicit "NONE".
pub fn override_option_with_option_none(old: &mut Option<String>, new: Option<String>) {
    if let Some(new) = new {
        if new == "NONE" {
            *old = None;
        } else {
            *old = Some(new);
        }
    }
}

/// when the current value is a concrete value and needs to be overridden if a new value is
/// present
pub fn override_if_present<T>(old: &mut T, new: Option<T>) {
    if let Some(new) = new {
        *old = new;
    }
}
