use crate::bluesky::domain::dto::access_token_dto::AccessTokenDto;

pub fn create_header(access_token: &AccessTokenDto, content_type: &str) -> Vec<String> {
    let token: &str = access_token.access_token.as_str();
    let auth_header: String = format!("Authorization: Bearer {token}");
    let content_type_header = format!("Content-Type: {content_type}");
    vec![auth_header, content_type_header]
}
