// mod infrastructure {
//     pub mod bluesky {
//         pub trait MessageSender {}
//     }
// }
//
// use infrastructure::bluesky::MessageSender;

mod infrastructure;
mod domain;

use infrastructure::bluesky::BlueSky;
use infrastructure::bluesky::MessageSender as BlueSkyMessageSender;
use infrastructure::mastodon::MessageSender as MastodonMessageSender;
use infrastructure::json_handler::JsonHandler;

fn main() {
    // println!("Hello, world!");
    let json_handler: &dyn JsonHandler = &JsonHandler{};
    json_handler.parse_json();
    let sender: BlueSky = BlueSkyMessageSender{};
    sender.send_message();
}

// application
// fn read_json() {
// }

// fn send_message() {
// }

// domain
// fn login() {
// }

// #[derive(Serialize, Deserialize)]
// struct Message {
//     content: String,
//     sender: String,
//     receivers: Receivers,
// }
//
// enum Receivers {
//     BlueSky,
//     Mastodon,
// }

// domain_service
// fn json_to_message() -> Result<Message, serde_json::Error> {
//     let json = r#"{"content": "Hello, world!", "sender": "Alice", "receivers": "BlueSky"}"#;
//     let message: Message = serde_json::from_str(json)?;
//
//     Ok(message)
// }
