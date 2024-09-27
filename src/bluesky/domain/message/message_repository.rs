use crate::bluesky::domain::login::model::access_token::AccessToken;
use crate::bluesky::domain::message::model::commit_message::CommitMessage;

pub trait MessageRepository {
    fn send(access_token: &AccessToken, post_data: &CommitMessage) -> Result<bool, curl::Error>;
}
