use crate::bluesky::domain::dto::access_token_dto::AccessTokenDto;
use crate::bluesky::domain::http::create_header;
use crate::util::set_headers;
use curl::easy::Easy;
use crate::bluesky::domain::message::message_repository::MessageRepository;
use crate::bluesky::infrastructure::message_repository_impl::MessageRepositoryImpl;

pub fn send_message(access_token: &AccessTokenDto) -> Result<bool, curl::Error> {
    let post_data = set_post_message(access_token);
    MessageRepositoryImpl::send(access_token, post_data);
    // let mut response_data = Vec::new();
    // let mut curl = Easy::new();
    // curl.url("https://bsky.social/xrpc/com.atproto.repo.createRecord")
    //     .unwrap();
    // curl.post(true).unwrap();
    //
    // let headers = create_header(access_token, "application/json");
    // let header_list = set_headers(headers);
    // curl.http_headers(header_list).unwrap();
    //
    // let post_data = set_post_message(access_token);
    // let binding = serde_json::to_string(&post_data).unwrap();
    // let serialized = binding.as_bytes();
    // println!(
    //     "POST data: {:?}",
    //     String::from_utf8(serialized.to_vec()).unwrap()
    // );
    //
    // curl.post_fields_copy(serialized).unwrap();
    // {
    //     let mut transfer = curl.transfer();
    //     transfer
    //         .write_function(|data| {
    //             response_data.extend_from_slice(data);
    //             Ok(data.len())
    //         })
    //         .unwrap();
    //     transfer.perform().unwrap();
    // }
    // let res_string = String::from_utf8(response_data).unwrap();
    // println!("{}", res_string);
    // Ok(true)
}
