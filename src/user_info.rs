use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    name: String,
    nb: String
}

impl UserInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nb(&self) -> &str {
        &self.nb
    }
}