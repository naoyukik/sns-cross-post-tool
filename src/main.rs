mod bluesky;

use chrono::Utc;
use curl::easy::List;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;
use crate::bluesky::{login, send_message};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode: String = if args.len() == 2 {
        args[1].clone()
    } else {
        String::from("")
    };
    if mode != "send" {
        exit(0)
    }

    // send message
    // let access_token = login();
    match login() {
        Ok(token) => {
            match send_message(&token) {
                Ok(_) => println!("Message has been sent successfully."),
                Err(err) => println!("Failed to send the message: {:?}", err),
            };
        }
        Err(err) => {
            println!("Login failed.: {:?}", err)
        }
    }
}

pub fn read_json_file(file_path: &str) -> Result<Message, Error> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    let json_object: Message = serde_json::from_reader(reader)?;

    Ok(json_object)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    content: String,
    sender: String,
    receivers: Vec<Receivers>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Receivers {
    BlueSky,
    Mastodon,
}

pub fn set_headers(header_list: Vec<String>) -> List {
    let mut headers = List::new();
    for header in header_list {
        headers.append(header.as_str()).unwrap();
    }
    headers
}

pub fn get_current_time() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub struct AccessToken {
    access_token: String,
}
