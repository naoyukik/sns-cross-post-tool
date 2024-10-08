use crate::bluesky::application::dto::access_token_dto::AccessToken;
use crate::bluesky::application::dto::commit_message_dto::{CommitMessageDto, CommitMessageRecordDto};
use crate::bluesky::application::dto::embed_dto::EmbedDto;
use crate::bluesky::application::dto::facet_dto::FacetDto;
use crate::bluesky::domain::env::env_repository::EnvRepository;
use crate::bluesky::domain::website_card_embeds;
use crate::bluesky::domain::website_card_embeds::website_card_embeds_service::{
    create_website_card_embeds, UploadedImageBlobDto,
};
use crate::bluesky::infrastructure::env_repository_impl::EnvRepositoryImpl;
use crate::{get_current_time, ogp_scraping, read_json_file};
use serde::de::Unexpected::Option;
use crate::bluesky::domain::message::facet_service::create_facets;

pub fn set_post_message(access_token: &AccessToken) -> CommitMessageDto {
    let account = EnvRepositoryImpl::get_login_credential("./".to_string());
    let message = read_json_file("message.json").unwrap();
    let content_with_fixed_hashtags =
        format!("{} {}", message.content, message.fixed_hashtags.bluesky);
    let cloned_content = content_with_fixed_hashtags.clone();
    let facets = create_facets(&cloned_content);

    let url_string = get_url_string(&cloned_content);
    let mut embed: Option<EmbedDto> = None;
    if !url_string.is_empty() {
        let ogp = ogp_scraping::fetch_ogp_data(url_string); // expect(&format!("Error occurred: Error"));
        if let Ok(ogp) = ogp {
            embed = Some(create_website_card_embeds(access_token, &ogp));
        }
    }
    let text_entry = CommitMessageRecordDto {
        text: content_with_fixed_hashtags,
        created_at: get_current_time(),
        facets,
        _type: "app.bsky.feed.post".to_string(),
        embed,
    };
    CommitMessageDto {
        repo: account.identifier,
        collection: "app.bsky.feed.post".to_string(),
        record: text_entry,
    }
}

fn get_url_string(text: &str) -> String {
    let matches = find_link_string(text);
    println!("matches: {:?}", matches);
    let mut url = "";
    for caps in matches {
        if let Some(cap) = caps.get(2) {
            url = cap.as_str();
        }
    }
    url.to_string()
}
