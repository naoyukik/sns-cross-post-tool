use crate::bluesky::domain::dto::access_token_dto::AccessTokenDto;
use crate::bluesky::domain::dto::commit_message_dto::CommitMessageDto;

pub trait MessageRepository {
    fn send(access_token: &AccessTokenDto, post_data: &CommitMessageDto) -> Result<bool, curl::Error>;
}
