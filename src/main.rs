mod bluesky {
    pub mod application;
    pub mod domain;
    pub mod infrastructure;
    pub mod presentation;
    pub mod util;
}
mod mastodon {
    pub mod application;
    pub mod domain;
    pub mod infrastructure;
    pub mod presentation;
    pub mod util;
}
mod ogp;
mod ogp_scraping;
mod util;

use crate::bluesky::presentation::message_resolver::post;
use crate::mastodon::presentation::message_resolver::post as mPost;
use crate::util::read_json_file;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use std::env;


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
            Receivers::Bluesky => match post() {
                Ok(_) => println!("Bluesky: Message has been sent successfully."),
                Err(err) => println!("Bluesky: Failed to send the message: {:?}", err),
            },
            Receivers::Mastodon => match mPost() {
                Ok(_) => println!("Mastodon: Message has been sent successfully."),
                Err(err) => println!("Mastodon: Failed to send the message: {:?}", err),
            }
        }
    }
}

// pub fn read_json_file(file_path: &str) -> Result<Message, Error> {
//     let file = File::open(file_path).expect("File not found");
//     let reader = BufReader::new(file);
// 
//     let json_object: Message = serde_json::from_reader(reader)?;
// 
//     Ok(json_object)
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    content: String,
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
    Bluesky,
    Mastodon,
}

pub fn response_to<T: DeserializeOwned>(response_data: Vec<u8>) -> T {
    let res_string = String::from_utf8(response_data).unwrap();
    println!("{}", res_string);
    let sliced_res = res_string.as_str();
    serde_json::from_str::<T>(sliced_res).unwrap()
}
