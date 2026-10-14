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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_u32_from_string_should_succeed(){
        let user: UserInfo = serde_json::from_str(r#"{"name":"bob","nb":"42"}"#).unwrap();
        assert_eq!(user.nb, 42);
    }

    #[test]
    fn deserialize_u32_from_string_should_fail_on_invalid_number(){
        assert!(serde_json::from_str::<UserInfo>(r#"{"name":"bob","nb":"abc"}"#).is_err());
    }

    #[test]
    fn deserialize_u32_from_string_should_fail_on_negative_number(){
        assert!(serde_json::from_str::<UserInfo>(r#"{"name":"bob","nb":"-1"}"#).is_err());
    }

    #[test]
    fn deserialize_u32_from_string_should_fail_on_overflow(){
        let json = format!(r#"{{"name":"bob","nb":"{}"}}"#, u64::from(u32::MAX) + 1);
        assert!(serde_json::from_str::<UserInfo>(&json).is_err());
    }
}