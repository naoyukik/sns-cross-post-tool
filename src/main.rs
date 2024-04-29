use crate::bluesky::{login, send_message};
use chrono::Utc;
use curl::easy::List;
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

mod bluesky {
    use crate::{get_current_time, read_json_file, set_headers, AccessToken};
    use curl::easy::Easy;
    use regex::{Captures, Match, Regex};
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
        #[serde(rename = "createdAt")]
        created_at: String,
        facets: Vec<Facet>,
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
        println!("{}", res_json);
        Ok(AccessToken {
            access_token: res_json["accessJwt"].to_string().replace('\"', ""),
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
        let cloned_content = message.content.clone();
        CommitMessage {
            repo: account.identifier,
            collection: "app.bsky.feed.post".to_string(),
            record: TextEntry {
                text: message.content,
                created_at: get_current_time(),
                facets: create_tags_facets(&cloned_content),
            },
        }
    }

    fn create_header(access_token: &AccessToken) -> Vec<String> {
        let token: &str = access_token.access_token.as_str();
        println!("Authorization: Bearer {}", token);
        let auth_header: String = format!("Authorization: Bearer {}", token);
        vec![auth_header, "Content-Type: application/json".to_string()]
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct FacetIndex {
        #[serde(rename = "byteStart")]
        byte_start: u16,
        #[serde(rename = "byteEnd")]
        byte_end: u16,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct FacetFeatures {
        #[serde(rename = "$type")]
        facet_type: String,
        tag: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Facet {
        index: FacetIndex,
        features: Vec<FacetFeatures>,
    }

    fn tags_to_facet_features(tag: &str) -> Vec<FacetFeatures> {
        let re = Regex::new(r"^#").unwrap();
        vec![FacetFeatures {
            facet_type: "app.bsky.richtext.facet#tag".to_string(),
            tag: re.replace(tag.trim(), "").to_string(),
        }]
    }

    fn create_tags_facets(message_content: &str) -> Vec<Facet> {
        find_hash_tags(message_content)
            .iter()
            .filter_map(|capture| capture.get(2).map(to_facet))
            .collect()
    }

    fn find_hash_tags(haystack: &str) -> Vec<Captures> {
        let pattern = r"(^|\s)(#\w*)";
        let regex_pattern = Regex::new(pattern).unwrap();
        regex_pattern.captures_iter(haystack).collect()
    }

    fn to_facet(capture_group: Match) -> Facet {
        let start = capture_group.start() as u16;
        let end = capture_group.end() as u16;
        let tag = capture_group.as_str();
        Facet {
            index: to_facet_index(&start, &end),
            features: tags_to_facet_features(tag),
        }
    }

    fn to_facet_index(start: &u16, end: &u16) -> FacetIndex {
        FacetIndex {
            byte_start: *start,
            byte_end: *end,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::bluesky::{get_profile, set_post_message};
        use crate::Receivers;
        use curl::easy::Easy;
        use std::io::{stdout, Write};

        #[test]
        fn can_create_tags() {
            let index1 = FacetIndex {
                byte_start: 29,
                byte_end: 34,
            };
            let features1 = vec![{
                FacetFeatures {
                    facet_type: String::from("app.bsky.richtext.facet#tag"),
                    tag: String::from("test"),
                }
            }];
            let index2 = FacetIndex {
                byte_start: 35,
                byte_end: 41,
            };
            let features2 = vec![{
                FacetFeatures {
                    facet_type: String::from("app.bsky.richtext.facet#tag"),
                    tag: String::from("test2"),
                }
            }];
            let expected = vec![
                Facet {
                    index: index1,
                    features: features1,
                },
                Facet {
                    index: index2,
                    features: features2,
                },
            ];
            let sut = create_tags_facets("ハッシュ投稿テスト\n\n#test #test2");
            println!("{:?}", sut);
            assert_eq!(
                sut.first().unwrap().index.byte_start,
                expected.first().unwrap().index.byte_start
            );
            assert_eq!(
                sut.first().unwrap().index.byte_end,
                expected.first().unwrap().index.byte_end
            );
            assert_eq!(
                sut.first().unwrap().features.first().unwrap().tag,
                expected.first().unwrap().features.first().unwrap().tag
            );
            assert_eq!(
                sut.get(1).unwrap().index.byte_start,
                expected.get(1).unwrap().index.byte_start
            );
            assert_eq!(
                sut.get(1).unwrap().index.byte_end,
                expected.get(1).unwrap().index.byte_end
            );
            assert_eq!(
                sut.get(1).unwrap().features.first().unwrap().tag,
                expected.get(1).unwrap().features.first().unwrap().tag
            );
        }

        #[test]
        fn can_find_hash_tags() {
            let hashes = [
                ["#test", "29", "34"],
                ["#日本語のテスト", "35", "57"],
                ["#1111", "58", "63"],
            ];

            let matches = find_hash_tags("ハッシュ投稿テスト\n\n#test #日本語のテスト #1111");
            for (hash_index, caps) in matches.iter().enumerate() {
                if let Some(captures) = caps.get(2) {
                    println!("{}", hash_index);
                    match hashes.get(hash_index) {
                        Some(hash) => {
                            assert_eq!(&captures.as_str(), hash.first().unwrap());
                            assert_eq!(
                                captures.start().to_string(),
                                hash.get(1).unwrap().to_string()
                            );
                            assert_eq!(
                                captures.end().to_string(),
                                hash.get(2).unwrap().to_string()
                            );
                        }
                        None => panic!("No hash found at index 0"),
                    }
                }
            }
        }

        #[test]
        fn learn_find_hash_tags() {
            let text = "ハッシュ投稿テスト\n\n#test #test2 #3test #1111 #日本語のカタカナ。";
            // let pattern = r"(^|\s)(#[^\d\s]\w*)";
            // let regex_pattern = Regex::new(pattern).unwrap();
            //
            // let matches = regex_pattern.find_iter(text);
            let matches = find_hash_tags(text);
            println!("matches: {:?}", matches);
            for caps in matches {
                if let Some(cap) = caps.get(2) {
                    println!("Matched: {}", cap.as_str());
                    println!("Start: {}", cap.start());
                    println!("End: {}", cap.end());
                }
            }
        }

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
}
