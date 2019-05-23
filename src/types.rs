use reqwest::Url;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    Generic {
        #[serde(deserialize_with = "deserialize_url")]
        #[serde(serialize_with = "serialize_url")]
        url: Url,
    },
    Hastebin {
        #[serde(deserialize_with = "deserialize_url")]
        #[serde(serialize_with = "serialize_url")]
        url: Url,
    },
}

fn deserialize_url<'de, D>(d: D) -> Result<Url, D::Error>
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

fn serialize_url<S>(x: &Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(x.as_str())
}
