use std::io;

pub fn ask_file_path() -> String {
    let mut user_path = String::new();
    println!("Enter the json's file name in data's folder (ex: example-data.json)");
    io::stdin().read_line(&mut user_path).expect("Failed to read line");
    return user_path;
}