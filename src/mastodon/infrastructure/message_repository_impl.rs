use crate::mastodon::domain::env::model::access_token::AccessToken;
use crate::mastodon::domain::message::message_repository::MessageRepository;
use crate::mastodon::domain::message::model::commit_message::CommitMessage;
use crate::mastodon::util::http::create_header;
use crate::util::set_headers;
use curl::easy::Easy;
use url::Url;

pub struct MessageRepositoryImpl {}

impl MessageRepository for MessageRepositoryImpl {
    fn send(access_token: &AccessToken, endpoint: &Url, post_data: &CommitMessage) -> Result<bool, curl::Error> {
        let mut curl = Easy::new();
        curl.url(endpoint.as_str())?;
        curl.post(true)?;

        let content_type = "application/json";
        let headers = create_header(access_token, content_type);
        let header_list = set_headers(headers);
        curl.http_headers(header_list)?;

        let binding = serde_json::to_string(&post_data).unwrap();
        let serialized = binding.as_bytes();
        println!(
            "POST data: {:?}",
            String::from_utf8(serialized.to_vec()).unwrap()
        );
        curl.post_fields_copy(serialized)?;

        let mut response_data = Vec::new();
        {
            let mut transfer = curl.transfer();
            transfer
                .write_function(|data| {
                    response_data.extend_from_slice(data);
                    Ok(data.len())
                })?;
            transfer.perform()?;
        }
        let res_string = String::from_utf8(response_data).unwrap();
        println!("{}", res_string);
        Ok(true)
    }
}
