mod bluesky {
    pub mod application;
    pub mod domain;
    pub mod infrastructure;
    pub mod presentation;
    pub mod util;
}
mod mastodon;
mod ogp;
mod ogp_scraping;
mod util;

use crate::bluesky::presentation::message_resolver::post;
use crate::mastodon::send_message;
use curl::easy::List;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

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

    let message = read_json_file("message.json").unwrap();

    for receiver in message.receivers {
        match receiver {
            Receivers::BlueSky => match post() {
                Ok(_) => println!("Bluesky: Message has been sent successfully."),
                Err(err) => println!("Bluesky: Failed to send the message: {:?}", err),
            },
            Receivers::Mastodon => {
                let config = mastodon::set_config();
                let api_client = mastodon::ApiClient { config };
                match mastodon::send_message(api_client) {
                    Ok(_) => println!("Mastodon: Message has been sent successfully."),
                    Err(err) => println!("Mastodon: Failed to send the message: {:?}", err),
                }
            }
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
    fixed_hashtags: FixedHashtags,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct FixedHashtags {
    mastodon: String,
    bluesky: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Receivers {
    BlueSky,
    Mastodon,
}

// TODO Delete the reference when you are done with mastodon.
pub fn set_headers(header_list: Vec<String>) -> List {
    let mut headers = List::new();
    for header in header_list {
        headers.append(header.as_str()).unwrap();
    }
    headers
}

pub fn response_to<T: DeserializeOwned>(response_data: Vec<u8>) -> T {
    let res_string = String::from_utf8(response_data).unwrap();
    println!("{}", res_string);
    let sliced_res = res_string.as_str();
    serde_json::from_str::<T>(sliced_res).unwrap()
}

// TODO Delete the reference when you are done with mastodon.
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    access_token: String,
}
