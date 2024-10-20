use std::fs::File;
use std::io::BufReader;
use chrono::Utc;
use curl::easy::List;
use regex::{Captures, Regex};
use serde_json::Error;
use crate::Message;

pub fn get_current_time() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn find_hash_tags(haystack: &str) -> Vec<Captures> {
    let pattern = r"(^|\s)(#\w*)";
    let regex_pattern = Regex::new(pattern).unwrap();
    regex_pattern.captures_iter(haystack).collect()
}

pub fn find_link_string(haystack: &str) -> Vec<Captures> {
    let pattern = r"(^|\s)(https?://(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))";
    let regex = Regex::new(pattern).unwrap();
    regex.captures_iter(haystack).collect()
}

pub fn set_headers(header_list: Vec<String>) -> List {
    let mut headers = List::new();
    for header in header_list {
        headers.append(header.as_str()).unwrap();
    }
    headers
}

pub fn read_json_file(file_path: &str) -> Result<Message, Error> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    let json_object: Message = serde_json::from_reader(reader)?;

    Ok(json_object)
}
