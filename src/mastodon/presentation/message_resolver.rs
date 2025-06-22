use crate::mastodon::application::authentication_workflow::login;
use crate::mastodon::application::send_message_workflow::send_message;
use crate::shared::domain::message::model::merged_message::MergedMessage;

pub fn post(merged_message: &MergedMessage) -> Result<bool, curl::Error> {
    let config = login();
    let result = send_message(&config, merged_message)?;
    Ok(result)
}
