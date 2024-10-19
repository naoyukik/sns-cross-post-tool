use crate::mastodon::domain::env::model::access_token::AccessToken;

pub fn create_header(access_token: &AccessToken, content_type: &str) -> Vec<String> {
    let token: &str = access_token.value();
    let auth_header: String = format!("Authorization: Bearer {token}");
    let content_type_header = format!("Content-Type: {content_type}");
    vec![auth_header, content_type_header]
}
