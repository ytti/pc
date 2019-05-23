use serde::{Deserialize, Deserializer, Serializer};
use url::Url;

pub(crate) fn deserialize_url<'de, D>(d: D) -> Result<Url, D::Error>
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

pub(crate) fn serialize_url<S>(x: &Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(x.as_str())
}
