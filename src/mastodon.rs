use crate::mastodon::credential_account::CredentialAccountRoot;
use crate::{read_json_file, response_to, set_headers, AccessToken};
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct CommitMessage {
    status: String,
}

struct Endpoints {
    search: &'static str,
    statuses: &'static str,
}

impl Endpoints {
    fn new() -> Self {
        Self {
            search: "/api/v1/accounts/verify_credentials",
            statuses: "/api/v1/statuses",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    domain: String,
    access_token: AccessToken,
    account: String,
}

trait EndpointBuilder {
    fn base_url(&self) -> Url;
    fn create_endpoint_url(&self, path: &str) -> Url;
}

pub struct ApiClient {
    pub config: Config,
}

impl EndpointBuilder for ApiClient {
    fn base_url(&self) -> Url {
        Url::parse(&format!("https://{}", self.config.domain)).expect("Failed Parse URL")
    }
    fn create_endpoint_url(&self, path: &str) -> Url {
        assert!(path.starts_with('/'), "Path must start with `/`");
        self.base_url()
            .join(path)
            .expect("Failed create endpoint url")
    }
}

pub fn set_config() -> Config {
    dotenvy::dotenv().expect("Failed to load .env file");
    let domain =
        env::var("MASTODON_DOMAIN").expect("Please set the MASTODON_DOMAIN environment variable");
    let password = env::var("MASTODON_APP_PASSWORD")
        .expect("Please set the MASTODON_APP_PASSWORD environment variable");
    let account =
        env::var("MASTODON_ACCOUNT").expect("Please set the MASTODON_ACCOUNT environment variable");

    let access_token = AccessToken {
        access_token: password,
    };

    Config {
        domain,
        access_token,
        account,
    }
}

fn fetch_user_account(api_client: ApiClient) -> CredentialAccountRoot {
    let endpoints = Endpoints::new();
    let endpoint = api_client.create_endpoint_url(endpoints.search);
    let queries = vec![
        ("q", api_client.config.account),
        ("type", "accounts".to_string()),
    ];
    let url_with_params = Url::parse_with_params(endpoint.as_str(), queries).unwrap();

    let mut curl = Easy::new();
    curl.url(url_with_params.as_str()).unwrap();

    let headers = create_header(&(api_client.config.access_token));
    let header_list = set_headers(headers);
    curl.http_headers(header_list).unwrap();

    let mut response_data = Vec::new();
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

    response_to::<CredentialAccountRoot>(response_data)
}

mod credential_account {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CredentialAccountRoot {
        acct: String,
        avatar: String,
        avatar_static: String,
        avatar_thumbhash: String,
        bot: bool,
        cat: bool,
        created_at: String,
        discoverable: Option<bool>,
        display_name: String,
        emojis: Vec<String>,
        fields: Vec<Fields>,
        followers_count: i32,
        following_count: i32,
        group: bool,
        header: String,
        header_static: String,
        pub id: String,
        last_status_at: Option<String>,
        locked: bool,
        note: String,
        other_settings: OtherSettings,
        searchability: String,
        source: Source,
        statuses_count: i32,
        subscribing_count: i32,
        url: String,
        username: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Fields {
        name: String,
        value: String,
        verified_at: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct OtherSettings {
        cat_ears_color: String,
        enable_reaction: bool,
        hide_followers_count: bool,
        hide_following_count: bool,
        hide_network: bool,
        hide_statuses_count: bool,
        location: String,
        noindex: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Source {
        fields: Vec<Fields>,
        follow_requests_count: i32,
        language: String,
        note: String,
        privacy: String,
        sensitive: bool,
    }
}

fn create_header(access_token: &AccessToken) -> Vec<String> {
    let token: &str = access_token.access_token.as_str();
    println!("Authorization: Bearer {}", token);
    let auth_header: String = format!("Authorization: Bearer {}", token);
    vec![auth_header, "Content-Type: application/json".to_string()]
}

fn to_user_account(response: CredentialAccountRoot) -> UserAccount {
    UserAccount {
        account_id: response.id,
    }
}

fn set_post_message() -> CommitMessage {
    let message = read_json_file("message.json").unwrap();
    let content_with_fixed_hashtags = format!("{} {}", message.content, message.fixed_hashtags.mastodon);
    CommitMessage {
        status: content_with_fixed_hashtags,
    }
}

struct UserAccount {
    account_id: String,
}

pub fn send_message(api_client: ApiClient) -> Result<bool, curl::Error> {
    let endpoints = Endpoints::new();
    let endpoint = api_client.create_endpoint_url(endpoints.statuses);

    let mut curl = Easy::new();
    curl.url(endpoint.as_str()).unwrap();
    curl.post(true).unwrap();

    let headers = create_header(&(api_client.config.access_token));
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

    let mut response_data = Vec::new();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learn_mastodon_message() {
        let post_message = set_post_message();
        print!("{:?}", post_message);
    }

    #[test]
    fn learn_search_user_id() {
        let config = set_config();
        let api_client = ApiClient { config };
        fetch_user_account(api_client);
    }

    // #[test]
    // fn learn_post_message() {
    //     let config = set_config();
    //     let api_client = ApiClient { config };
    //     send_message(api_client);
    // }
}
