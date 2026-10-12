use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    name: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    nb: u64
}

fn deserialize_u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}