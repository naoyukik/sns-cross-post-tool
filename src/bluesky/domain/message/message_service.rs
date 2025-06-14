use crate::bluesky::domain::env::env_repository::EnvRepository;
use crate::bluesky::domain::login::model::access_token::AccessToken;
use crate::bluesky::domain::message::facet_service::create_facets;
use crate::bluesky::domain::message::model::commit_message::{CommitMessage, CommitMessageRecord};
use crate::bluesky::domain::website_card_embeds::website_card_embeds_service::create_website_card_embeds;
use crate::bluesky::infrastructure::env_repository_impl::EnvRepositoryImpl;
use crate::ogp_scraping;
use crate::shared::domain::message::model::merged_message::MergedMessage;
use crate::shared::domain::message_service::{MessageService, MessageServiceImpl};
use crate::shared::domain::time_service::{TimeService, TimeServiceImpl};

pub fn set_post_message(
    access_token: &AccessToken,
    merged_message: &MergedMessage,
) -> CommitMessage {
    let account = EnvRepositoryImpl::get_login_credential("./.env".to_string());
    let content_with_fixed_hashtags = format!(
        "{} {}",
        merged_message.content, merged_message.fixed_hashtags.bluesky
    );
    let cloned_content = content_with_fixed_hashtags.clone();
    let facets = create_facets(&cloned_content);

    let url_string = get_url_string(&cloned_content);
    let mut embed = None;
    if !url_string.is_empty() {
        let ogp = ogp_scraping::fetch_ogp_data(url_string); // expect(&format!("Error occurred: Error"));
        if let Ok(ogp) = ogp {
            embed = create_website_card_embeds(access_token, &ogp);
        }
    }
    let time_service = TimeServiceImpl;
    let record = CommitMessageRecord {
        text: content_with_fixed_hashtags,
        created_at: time_service.get_current_time(),
        facets,
        _type: "app.bsky.feed.post".to_string(),
        embed,
    };
    CommitMessage {
        repo: account.get_identifier(),
        collection: "app.bsky.feed.post".to_string(),
        record,
    }
}

fn get_url_string(text: &str) -> String {
    let matches = MessageServiceImpl::find_link_string(text);
    debug!("matches: {:?}", matches);
    let mut url = "";
    for caps in matches {
        if let Some(cap) = caps.get(2) {
            url = cap.as_str();
        }
    }
    url.to_string()
}
