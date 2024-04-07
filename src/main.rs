use serde_json;
use serde::{Deserialize, Serialize};


fn main() {
    println!("hello world");
}

pub fn learn_main() -> bool {
    true
}

// pub fn read_json_file() -> String {
//     let path = "message.json";
//     let file = File::open(path).expect("File not found");
//     let mut reader = BufReader::new(file);
//     let mut body = String::new();
//
//     reader.read_to_string(&mut body).expect("Failed to read file");
//
//     body
// }

pub fn parse_json(json_string: &str) -> Message {
    let parsed: Message = serde_json::from_str(&json_string)
        .expect("Couldn't parse JSON string");
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

    // test tasks
    // Can read a JSON file and convert it into a Message type
    #[test]
    fn can_parse_json() {
        let json_string = r#"
        {
            "content": "Test message",
            "sender": "user1",
            "receivers": ["BlueSky"]
        }
        "#;
        let parsed: Message = parse_json(json_string);
        assert_eq!(parsed.content, "Test message");
        assert_eq!(parsed.sender, "user1");
        assert_eq!(parsed.receivers.len(), 1);
        assert!(parsed.receivers.contains(&Receivers::BlueSky));
    }
    // fn can_read_json_file() {
    //     assert_eq!("true", read_json_file())
    // }
    // fn can_read_convert_message_type_from_json() {}

    // Can 'login' to bluesky
    // Can post to bluesky
    // Failed to load JSON file
    // Failed to convert JSON file into Message type
    // Failed to log in
    // Retrieve error when failed to post
}
