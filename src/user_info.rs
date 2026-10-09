use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    name: String,
    nb: String
}