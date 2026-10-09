use std::{fs, io};

use crate::user_info::UserInfo;

pub fn import_json_file(file_name: &str) -> Result<Vec<UserInfo>, io::Error> {
    let path = format!("data/{}.json", file_name.trim());
    // referece of path to not move
    let content = fs::read_to_string(&path)?;
    let collections = serde_json::from_str(&content)?;
    return Ok(collections);
}