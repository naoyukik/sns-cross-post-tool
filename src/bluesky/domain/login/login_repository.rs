use crate::bluesky::domain::dto::access_token_dto::AccessTokenDto;

pub trait LoginRepository {
    fn login() -> Result<AccessTokenDto, curl::Error>;
}
