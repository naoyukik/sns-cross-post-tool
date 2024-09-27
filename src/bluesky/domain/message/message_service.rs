use crate::bluesky::domain::dto::access_token_dto::AccessTokenDto;
use crate::{get_current_time, ogp_scraping, read_json_file};
use crate::bluesky::domain::dto::commit_message_dto::CommitMessageDto;
use crate::bluesky::domain::website_card_embeds;

fn set_post_message(access_token: &AccessTokenDto) -> CommitMessageDto {
    let account = get_account();
    let message = read_json_file("message.json").unwrap();
    let content_with_fixed_hashtags =
        format!("{} {}", message.content, message.fixed_hashtags.bluesky);
    let cloned_content = content_with_fixed_hashtags.clone();
    let tags_facets = crate::bluesky::presentation::message_resolver::create_tags_facets(&cloned_content);
    let links_facets = crate::bluesky::presentation::message_resolver::create_links_facets(&cloned_content);
    let mut merged_facets: Vec<crate::bluesky::presentation::message_resolver::Facet> = tags_facets;
    merged_facets.extend(links_facets);

    let url_string = crate::bluesky::presentation::message_resolver::get_url_string(&cloned_content);
    let mut embed: Option<website_card_embeds::website_card_embeds::Embed> = None;
    if !url_string.is_empty() {
        let ogp = ogp_scraping::fetch_ogp_data(url_string); // expect(&format!("Error occurred: Error"));
        if let Ok(ogp) = ogp {
            embed = Some(website_card_embeds::website_card_embeds::Embed::create(access_token, &ogp));
        }
    }
    let text_entry = crate::bluesky::presentation::message_resolver::TextEntry {
        text: content_with_fixed_hashtags,
        created_at: get_current_time(),
        facets: merged_facets,
        _type: "app.bsky.feed.post".to_string(),
        embed,
    };
    crate::bluesky::presentation::message_resolver::CommitMessage {
        repo: account.identifier,
        collection: "app.bsky.feed.post".to_string(),
        record: text_entry,
    }
}
