use std::{fs, io};

use crate::user_info::UserInfo;

pub fn import_json_file(file_name: &str) -> Result<Vec<UserInfo>, io::Error> {
    let path = format!("data/{}.json", file_name.trim());
    // referece of path to not move
    let content = fs::read_to_string(&path)?;
    let collections = serde_json::from_str(&content)?;
    return Ok(collections);
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn import_json_file_should_succeed() {
        let file_name: String = String::from("data");   
        let mut is_loaded: bool = false;
        let mut collection: Vec<UserInfo> = Vec::new();   

        if let Ok(result) = import_json_file(&file_name) {
            is_loaded = true;
            collection = result;
        }

        assert!(is_loaded);
        assert_eq!(collection.len(), 21);
    }

    #[test]
    // the function return an error (ErrorKind::NotFound) because the file is not found
    fn import_json_file_should_return_error_not_found() {
        let file_name: String = String::from("no-such-file");

        let error = import_json_file(&file_name).expect_err("File shouldn't exist");
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    // the function return an error (ErrorKind::InvalidData) because the file doesn"t match the format
    fn import_json_file_should_return_error_cant_deserialize() {
        let file_name: String = String::from("data-wrong-format");

        let error = import_json_file(&file_name).expect_err("File shouldn't be in the right format");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }
}