use std::fs::File;
use std::io::{BufReader, Read};
use serde_json;
pub use crate::domain::json_handler::JsonHandler;
use crate::domain::message::Message;

impl dyn JsonHandler {
    pub fn parse_json(&self) -> Result<Message, serde_json::Error> {
        let json_file = Self::read_json();
        let message: Message = serde_json::from_str(&json_file)?;

        Ok(message)
    }

    fn read_json() -> String {
        let path = "message.json";
        let file = File::open(path).expect("File not found");
        let mut reader = BufReader::new(file);
        let mut content = String::new();

        reader.read_to_string(&mut content).expect("Failed to read file");

        content
    }
}
