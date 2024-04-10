use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Error, Value};
use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("hello world");
    // read json file
    // let file_path = "message.json";
    // let json_data = read_json_file(file_path).unwrap();
    // let message = parse_json(json_data);
    // parse json
    // SNS login
    // send message
}

pub fn learn_main() -> bool {
    true
}

pub fn read_json_file(file_path: &str) -> Result<Value, Error> {
    let file_path = file_path;
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    let json_object: Value = serde_json::from_reader(reader)?;

    Ok(json_object)
}

pub fn parse_json(json_string: &str) -> Message {
    let parsed: Message = serde_json::from_str(&json_string).expect("Couldn't parse JSON string");
    parsed
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    content: String,
    sender: String,
    receivers: Vec<Receivers>,
}

#[derive(Serialize, Deserialize, PartialEq)]
enum Receivers {
    BlueSky,
    Mastodon,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learn_main() {
        assert_eq!(true, learn_main());
    }

    #[test]
    fn learn_value() {
        let data = read_json_file("./tests/resources/message.json").unwrap();
        if let Some(sender) = data.get("sender") {
            println!("Sender -> {}", sender)
        } else {
            println!("Sender not found")
        }
    }

    // test tasks
    // Can read a JSON file and convert it into a Message type
    #[test]
    fn can_parse_json() {
        let json_string = r#"{
            "content": "Test message",
            "sender": "user1",
            "receivers": ["BlueSky"]
        }"#;
        let parsed: Message = parse_json(json_string);
        assert_eq!(parsed.content, "Test message");
        assert_eq!(parsed.sender, "user1");
        assert_eq!(parsed.receivers.len(), 1);
        assert!(parsed.receivers.contains(&Receivers::BlueSky));
    }

    #[test]
    fn can_read_json_file() {
        let left = r#"{
            "content": "Test message",
            "sender": "user1",
            "receivers": ["BlueSky"]
        }"#;
        assert_eq!(
            serde_json::from_str::<Value>(left).unwrap(),
            read_json_file("./tests/resources/message.json").unwrap()
        )
    }
    // fn can_read_convert_message_type_from_json() {}

    // Can 'login' to bluesky
    // Can post to bluesky
    // Failed to load JSON file
    // Failed to convert JSON file into Message type
    // Failed to log in
    // Retrieve error when failed to post
}
