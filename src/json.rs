use std::fs;

use crate::user_info::UserInfo;

pub fn import_json_file(file_name: &str) -> Vec<UserInfo> {
    let path = format!("data/{}.json", file_name.trim());
    // referece of path to not move
    let content = fs::read_to_string(&path);

    if let Ok(result) = content {
        let collections: Vec<UserInfo> = serde_json::from_str(&result).unwrap_or_else(|error|{
            panic!("Error while deserializing {error}");
        });
        return collections;
    }
    else {
        panic!("No file exist with the name: {}", &path)
    }

}