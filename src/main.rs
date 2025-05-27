extern crate env_logger;
#[macro_use]
extern crate log;
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

mod shared {
    pub mod domain;
}

mod ogp;
mod ogp_scraping;

use crate::bluesky::presentation::message_resolver::post;
use crate::mastodon::presentation::message_resolver::post as mPost;
use serde::de::DeserializeOwned;

use crate::shared::domain::message::model::message_template::Receivers;
use crate::shared::domain::message_service::{MessageService, MessageServiceImpl};
use clap::{Parser, Subcommand};
use std::process::exit;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Message to post on social media
    #[arg(short, long)]
    message: Option<String>,

    /// Execution mode
    #[command(subcommand)]
    command: Command,
}

impl Args {
    pub fn message(&self) -> &str {
        self.message.as_deref().unwrap_or("")
    }
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Execute the post
    Send,
}

fn main() {
    let args = Args::parse();
    let command = match args.command {
        Command::Send => "send",
    };

    if command != "send" {
        exit(0)
    }

    set_logger();

    let message = MessageServiceImpl::message_from_json_file("message.json").unwrap();

    for receiver in message.receivers {
        match receiver {
            Receivers::Bluesky => match post(&args) {
                Ok(_) => print!("Bluesky: Message has been sent successfully."),
                Err(err) => error!("Bluesky: Failed to send the message: {:?}", err),
            },
            Receivers::Mastodon => match mPost() {
                Ok(_) => print!("Mastodon: Message has been sent successfully."),
                Err(err) => error!("Mastodon: Failed to send the message: {:?}", err),
            },
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
//

pub fn response_to<T: DeserializeOwned>(response_data: Vec<u8>) -> T {
    let res_string = String::from_utf8(response_data).unwrap();
    debug!("{}", res_string);
    let sliced_res = res_string.as_str();
    serde_json::from_str::<T>(sliced_res).unwrap()
}

fn set_logger() {
    env_logger::init_from_env(env_logger::Env::new().filter("RUST_LOG"));
}

#[cfg(test)]
mod tests {
    use crate::set_logger;
    use dotenvy::dotenv;
    use std::env;

    #[test]
    fn learn_environments() {
        // Load environment variables from .env file
        dotenv().ok();

        // Print all environment variables
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn learn_set_logger() {
        set_logger();
        trace!("trace log");
        debug!("debug log");
        info!("info log");
        warn!("warn log");
        error!("error log");
    }
}
