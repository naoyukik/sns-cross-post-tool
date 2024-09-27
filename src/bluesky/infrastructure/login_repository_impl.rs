use crate::bluesky::domain::env::env_repository::EnvRepository;
use crate::bluesky::domain::login::login_repository::LoginRepository;
use crate::bluesky::domain::login::model::access_token::AccessToken;
use crate::bluesky::infrastructure::env_repository_impl::EnvRepositoryImpl;
use crate::util::set_headers;
use curl::easy::Easy;

pub struct LoginRepositoryImpl {}

impl LoginRepository for LoginRepositoryImpl {
    fn login() -> Result<AccessToken, curl::Error> {
        let mut response_data = Vec::new();
        let mut curl = Easy::new();
        curl.url("https://bsky.social/xrpc/com.atproto.server.createSession")?;
        curl.post(true)?;

        let headers = set_headers(vec!["Content-Type: application/json".to_string()]);
        curl.http_headers(headers)?;

        let post_data = EnvRepositoryImpl::get_login_credential("./.env".to_string());
        let binding = serde_json::to_string(&post_data).unwrap();
        let serialized = binding.as_bytes();
        println!(
            "POST data: {:?}",
            String::from_utf8(serialized.to_vec()).unwrap()
        );

        curl.post_fields_copy(serialized)?;
        {
            let mut transfer = curl.transfer();
            transfer.write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        let res_string = String::from_utf8(response_data).expect("Illegal JSON format");
        let sliced_res = res_string.as_str();
        let res_json: serde_json::Value = serde_json::from_str(sliced_res).unwrap();
        println!("login res_json {}", res_json);
        Ok(AccessToken::new(
            res_json["accessJwt"].to_string().replace('\"', ""),
        ))
    }
}

#[cfg(test)]
mod tests {}
