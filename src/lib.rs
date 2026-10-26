#![forbid(unsafe_code)]

use crate::trie::{insert, Node};
use crate::user_info::UserInfo;

pub mod json;
pub mod input;
pub mod user_info;
pub mod trie;
pub mod plantuml;

pub fn run(){
    let mut is_loaded: bool = false;
    let mut json_collection: Vec<UserInfo> = Vec::new();

    while !is_loaded {
        let file_name: String = input::ask_file_path();
        json_collection = match json::import_json_file(&file_name) {
            Ok(res) => {
                is_loaded = true;
                res
            },
            Err(e) => {
                eprintln!("Error while importing: {e}");
                continue;
            }
        };
    }

    let mut trees: Vec<Node> = Vec::new();
    for contact in &json_collection {
        insert(&mut trees, contact.nb(), contact.name());
    }

    println!("{trees:#?}");
}
