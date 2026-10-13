use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    name: String,
    #[serde(deserialize_with = "deserialize_u32_from_string")]
    nb: u32
}

fn deserialize_u32_from_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u32>().map_err(serde::de::Error::custom)
}