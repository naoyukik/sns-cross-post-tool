use crate::bluesky::{login, send_message};
use chrono::Utc;
use curl::easy::List;
use serde::{Deserialize, Serialize};
use serde_json;
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

    // send message
    let access_token = login();
    match access_token {
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
    let file_path = file_path;
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

mod bluesky {
    use crate::{get_current_time, read_json_file, set_headers, AccessToken};
    use curl::easy::Easy;
    use serde::{Deserialize, Serialize};
    use std::env;
    use url::Url;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct LoginCredentials {
        identifier: String,
        password: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CommitMessage {
        repo: String,
        collection: String,
        record: TextEntry,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TextEntry {
        text: String,
        createdAt: String,
    }

    pub fn login() -> Result<AccessToken, curl::Error> {
        let mut response_data = Vec::new();
        let mut curl = Easy::new();
        curl.url("https://bsky.social/xrpc/com.atproto.server.createSession")?;
        curl.post(true).unwrap();

        let headers = set_headers(vec!["Content-Type: application/json".to_string()]);
        curl.http_headers(headers).unwrap();

        let post_data = get_account();
        let binding = serde_json::to_string(&post_data).unwrap();
        let serialized = binding.as_bytes();
        println!(
            "POST data: {:?}",
            String::from_utf8(serialized.to_vec()).unwrap()
        );

        curl.post_fields_copy(serialized).unwrap();
        {
            let mut transfer = curl.transfer();
            transfer.write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        let res_string = String::from_utf8(response_data).unwrap();
        let sliced_res = res_string.as_str();
        let res_json: serde_json::Value = serde_json::from_str(sliced_res).unwrap();
        println!("{}", res_json.to_string());
        Ok(AccessToken {
            access_token: res_json["accessJwt"].to_string().replace("\"", ""),
        })
    }

    pub fn get_profile(access_token: &AccessToken) -> String {
        let mut response_data = Vec::new();
        let mut curl = Easy::new();
        let env = get_account();
        let queries = vec![("actor", env.identifier)];
        let url_with_params = Url::parse_with_params(
            "https://bsky.social/xrpc/app.bsky.actor.getProfile",
            queries,
        )
        .unwrap();
        curl.url(url_with_params.as_str()).unwrap();

        let headers = create_header(access_token);
        let header_list = set_headers(headers);
        curl.http_headers(header_list).unwrap();

        {
            let mut transfer = curl.transfer();
            transfer
                .write_function(|data| {
                    response_data.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        let res_string = String::from_utf8(response_data).unwrap();
        println!("{}", res_string);
        res_string
    }

    pub fn send_message(access_token: &AccessToken) -> Result<bool, curl::Error> {
        let mut response_data = Vec::new();
        let mut curl = Easy::new();
        curl.url("https://bsky.social/xrpc/com.atproto.repo.createRecord")
            .unwrap();
        curl.post(true).unwrap();

        let headers = create_header(access_token);
        let header_list = set_headers(headers);
        curl.http_headers(header_list).unwrap();

        let post_data = set_post_message();
        let binding = serde_json::to_string(&post_data).unwrap();
        let serialized = binding.as_bytes();
        println!(
            "POST data: {:?}",
            String::from_utf8(serialized.to_vec()).unwrap()
        );

        curl.post_fields_copy(serialized).unwrap();
        {
            let mut transfer = curl.transfer();
            transfer
                .write_function(|data| {
                    response_data.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        let res_string = String::from_utf8(response_data).unwrap();
        println!("{}", res_string);
        Ok(true)
    }

    fn get_account() -> LoginCredentials {
        let _ = dotenvy::dotenv().expect("Failed to load .env file");

        let identifier = env::var("BLUESKY_LOGIN_NAME")
            .expect("Please set the BLUESKY_LOGIN_NAME environment variable");
        let password = env::var("BLUESKY_APP_PASSWORD")
            .expect("Please set the BLUESKY_APP_PASSWORD environment variable");

        LoginCredentials {
            identifier,
            password,
        }
    }

    fn set_post_message() -> CommitMessage {
        let message = read_json_file("message.json").unwrap();
        let account = get_account();
        CommitMessage {
            repo: account.identifier,
            collection: "app.bsky.feed.post".to_string(),
            record: TextEntry {
                text: message.content,
                createdAt: get_current_time(),
            },
        }
    }

    fn create_header(access_token: &AccessToken) -> Vec<String> {
        let token: &str = access_token.access_token.as_str();
        println!("Authorization: Bearer {}", token);
        let auth_header: String = format!("Authorization: Bearer {}", token);
        vec![auth_header, "Content-Type: application/json".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bluesky::{get_profile, set_post_message};
    use curl::easy::Easy;
    use std::io::{stdout, Write};

    #[test]
    fn learn_bluesky_get_profile() {
        let access_token = login();

        // send message
        match access_token {
            Ok(token) => {
                get_profile(&token);
            }
            Err(err) => {
                println!("Login failed.: {:?}", err)
            }
        }
    }

    #[test]
    fn learn_bluesky_message() {
        set_post_message();
    }

    // #[test]
    // fn learn_bluesky_send_message() {
    //     let result = bluesky_send_message();
    //     assert_eq!(true, result)
    // }

    #[test]
    fn learn_value() {
        let data = read_json_file("./tests/resources/message.json").unwrap();
        println!("{}", data.sender)

        // if let Some(sender) = data.get("sender") {
        //     println!("Sender -> {}", sender)
        // } else {
        //     println!("Sender not found")
        // }
    }

    #[test]
    fn learn_env() {
        let _ = dotenvy::dotenv();

        // for (key, value) in env::vars() {
        //     println!("{key}: {value}");
        // }
        match env::var("BLUESKY_APP_PASSWORD") {
            Ok(val) => println!("{val:?}"),
            Err(e) => println!("err: {e}"),
        }
    }

    #[test]
    fn learn_curl() -> Result<(), curl::Error> {
        let mut curl = Easy::new();
        curl.url("https://bsky.social/xrpc/com.atproto.server.createSession")?;
        curl.write_function(|data| {
            stdout().write_all(data).unwrap();
            Ok(data.len())
        })?;
        curl.perform()?;
        Ok(())
    }

    #[test]
    fn can_read_json_file() {
        let result = read_json_file("./tests/resources/message.json").unwrap();
        assert_eq!(result.content, "Test message");
        assert_eq!(result.sender, "user1");
        assert_eq!(result.receivers.len(), 1);
        assert!(result.receivers.contains(&Receivers::BlueSky));
    }
}
