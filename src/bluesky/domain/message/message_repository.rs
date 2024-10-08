use crate::bluesky::application::dto::access_token_dto::AccessToken;
use crate::bluesky::application::dto::commit_message_dto::CommitMessageDto;

pub trait MessageRepository {
    fn send(
        access_token: &AccessToken,
        post_data: &CommitMessageDto,
    ) -> Result<bool, curl::Error>;
}
