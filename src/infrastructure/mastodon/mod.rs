pub struct Mastodon;

use crate::domain::message::Message;
pub use crate::domain::message_sender::MessageSender;

impl MessageSender for Mastodon {
    fn send_message(&self, message: Message) {
        println!("Mastodon: {}", message.get_content());
    }

    fn login(&self, username: &str, password: &str) -> bool {
        true
    }
}
