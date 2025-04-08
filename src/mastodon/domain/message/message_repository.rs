use crate::mastodon::domain::env::model::access_token::AccessToken;
use crate::mastodon::domain::message::model::commit_message::CommitMessage;
use url::Url;

pub trait MessageRepository {
    fn send(
        access_token: &AccessToken,
        endpoint: &Url,
        post_data: &CommitMessage,
    ) -> Result<bool, curl::Error>;
}
