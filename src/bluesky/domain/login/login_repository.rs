use crate::bluesky::domain::login::model::access_token::AccessToken;

pub trait LoginRepository {
    fn login() -> Result<AccessToken, curl::Error>;
}
