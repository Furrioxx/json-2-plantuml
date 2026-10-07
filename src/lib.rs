#![forbid(unsafe_code)]

use crate::user_info::UserInfo;

pub mod json;
pub mod input;
pub mod user_info;

pub fn run(){
    let file_name: String = input::ask_file_path();
    let json_collection: Vec<UserInfo> = json::import_json_file(&file_name);
    if json_collection.len() == 0 {
        panic!("the collection is empty");
    }
}