use crate::bluesky::domain::login::model::access_token::AccessToken;
use crate::bluesky::domain::message::message_repository::MessageRepository;
use crate::bluesky::domain::message::model::commit_message::CommitMessage;
use crate::bluesky::util::http::create_header;
use crate::util::set_headers;
use curl::easy::Easy;

pub struct MessageRepositoryImpl {}

impl MessageRepository for MessageRepositoryImpl {
    fn send(access_token: &AccessToken, post_data: &CommitMessage) -> Result<bool, curl::Error> {
        let mut curl = Easy::new();
        curl.url("https://bsky.social/xrpc/com.atproto.repo.createRecord")?;
        curl.post(true)?;

        let headers = create_header(access_token, "application/json");
        let header_list = set_headers(headers);
        curl.http_headers(header_list)?;

        let binding = serde_json::to_string::<CommitMessage>(&post_data).unwrap();
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
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        let res_string = String::from_utf8(response_data).unwrap();
        println!("{}", res_string);
        Ok(true)
    }
}
