use crate::{get_current_time, ogp, ogp_scraping, read_json_file, set_headers, AccessToken};
use curl::easy::Easy;
use regex::{Captures, Match, Regex};
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct LoginCredentials {
    identifier: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommitMessage {
    repo: String,
    collection: String,
    record: TextEntry,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextEntry {
    text: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    facets: Vec<Facet>,
    #[serde(rename = "$type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<website_card_embeds::Embed>,
}

pub fn login() -> Result<AccessToken, curl::Error> {
    let mut response_data = Vec::new();
    let mut curl = Easy::new();
    curl.url("https://bsky.social/xrpc/com.atproto.server.createSession")?;
    curl.post(true).unwrap();

    let headers = set_headers(vec!["Content-Type: application/json".to_string()]);
    curl.http_headers(headers).unwrap();

    let post_data = get_account();
    let binding = serde_json::to_string(&post_data).unwrap();
    let serialized = binding.as_bytes();
    println!(
        "POST data: {:?}",
        String::from_utf8(serialized.to_vec()).unwrap()
    );

    curl.post_fields_copy(serialized).unwrap();
    {
        let mut transfer = curl.transfer();
        transfer.write_function(|data| {
            response_data.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    let res_string = String::from_utf8(response_data).unwrap();
    let sliced_res = res_string.as_str();
    let res_json: serde_json::Value = serde_json::from_str(sliced_res).unwrap();
    println!("login res_json {}", res_json);
    Ok(AccessToken {
        access_token: res_json["accessJwt"].to_string().replace('\"', ""),
    })
}

pub fn get_profile(access_token: &AccessToken) -> String {
    let mut response_data = Vec::new();
    let mut curl = Easy::new();
    let env = get_account();
    let queries = vec![("actor", env.identifier)];
    let url_with_params = Url::parse_with_params(
        "https://bsky.social/xrpc/app.bsky.actor.getProfile",
        queries,
    )
    .unwrap();
    curl.url(url_with_params.as_str()).unwrap();

    let headers = create_header(access_token, "application/json");
    let header_list = set_headers(headers);
    curl.http_headers(header_list).unwrap();

    {
        let mut transfer = curl.transfer();
        transfer
            .write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    let res_string = String::from_utf8(response_data).unwrap();
    println!("{}", res_string);
    res_string
}

pub fn send_message(access_token: &AccessToken) -> Result<bool, curl::Error> {
    let mut response_data = Vec::new();
    let mut curl = Easy::new();
    curl.url("https://bsky.social/xrpc/com.atproto.repo.createRecord")
        .unwrap();
    curl.post(true).unwrap();

    let headers = create_header(access_token, "application/json");
    let header_list = set_headers(headers);
    curl.http_headers(header_list).unwrap();

    let post_data = set_post_message(access_token);
    let binding = serde_json::to_string(&post_data).unwrap();
    let serialized = binding.as_bytes();
    println!(
        "POST data: {:?}",
        String::from_utf8(serialized.to_vec()).unwrap()
    );

    curl.post_fields_copy(serialized).unwrap();
    {
        let mut transfer = curl.transfer();
        transfer
            .write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    let res_string = String::from_utf8(response_data).unwrap();
    println!("{}", res_string);
    Ok(true)
}

fn get_account() -> LoginCredentials {
    let _ = dotenvy::dotenv().expect("Failed to load .env file");

    let identifier = env::var("BLUESKY_LOGIN_NAME")
        .expect("Please set the BLUESKY_LOGIN_NAME environment variable");
    let password = env::var("BLUESKY_APP_PASSWORD")
        .expect("Please set the BLUESKY_APP_PASSWORD environment variable");

    LoginCredentials {
        identifier,
        password,
    }
}

fn set_post_message(access_token: &AccessToken) -> CommitMessage {
    let account = get_account();
    let message = read_json_file("message.json").unwrap();
    let content_with_fixed_hashtags = format!("{} {}", message.content, message.fixed_hashtags.bluesky);
    let cloned_content = content_with_fixed_hashtags.clone();
    let tags_facets = create_tags_facets(&cloned_content);
    let links_facets = create_links_facets(&cloned_content);
    let mut merged_facets: Vec<Facet> = tags_facets;
    merged_facets.extend(links_facets);

    let url_string = get_url_string(&cloned_content);
    let mut embed: Option<website_card_embeds::Embed> = None;
    if !url_string.is_empty() {
        let ogp = ogp_scraping::fetch_ogp_data(url_string); // expect(&format!("Error occurred: Error"));
        if let Ok(ogp) = ogp {
            embed = Some(website_card_embeds::create(access_token, &ogp));
        }
    }
    let text_entry = TextEntry {
        text: content_with_fixed_hashtags,
        created_at: get_current_time(),
        facets: merged_facets,
        _type: "app.bsky.feed.post".to_string(),
        embed,
    };
    CommitMessage {
        repo: account.identifier,
        collection: "app.bsky.feed.post".to_string(),
        record: text_entry,
    }
}

pub fn create_header(access_token: &AccessToken, content_type: &str) -> Vec<String> {
    let token: &str = access_token.access_token.as_str();
    let auth_header: String = format!("Authorization: Bearer {token}");
    let content_type_header = format!("Content-Type: {content_type}");
    vec![auth_header, content_type_header]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FacetIndex {
    #[serde(rename = "byteStart")]
    byte_start: u16,
    #[serde(rename = "byteEnd")]
    byte_end: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct FacetFeatures {
    #[serde(rename = "$type")]
    facet_type: String,
    #[serde(flatten)]
    feature_mode: FeatureMode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Facet {
    index: FacetIndex,
    features: Vec<FacetFeatures>,
}

fn tags_to_facet_features(tag: &str) -> Vec<FacetFeatures> {
    let re = Regex::new(r"^#").unwrap();
    vec![FacetFeatures {
        facet_type: "app.bsky.richtext.facet#tag".to_string(),
        feature_mode: FeatureMode::Tag(re.replace(tag.trim(), "").to_string()),
    }]
}

fn links_to_facet_features(tag: &str) -> Vec<FacetFeatures> {
    vec![FacetFeatures {
        facet_type: "app.bsky.richtext.facet#link".to_string(),
        feature_mode: FeatureMode::Uri(tag.trim().to_string()),
    }]
}

fn create_tags_facets(message_content: &str) -> Vec<Facet> {
    find_hash_tags(message_content)
        .iter()
        .filter_map(|capture| {
            capture
                .get(2)
                .map(|cap| to_facet(cap, FeatureMode::Tag("Tag".to_string())))
        })
        .collect()
}

fn create_links_facets(message_content: &str) -> Vec<Facet> {
    find_link_string(message_content)
        .iter()
        .filter_map(|capture| {
            capture
                .get(2)
                .map(|cap| to_facet(cap, FeatureMode::Uri("Uri".to_string())))
        })
        .collect()
}

fn find_hash_tags(haystack: &str) -> Vec<Captures> {
    let pattern = r"(^|\s)(#\w*)";
    let regex_pattern = Regex::new(pattern).unwrap();
    regex_pattern.captures_iter(haystack).collect()
}

fn find_link_string(haystack: &str) -> Vec<Captures> {
    let pattern = r"(^|\s)(https?://(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))";
    let regex = Regex::new(pattern).unwrap();
    regex.captures_iter(haystack).collect()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum FeatureMode {
    Tag(String),
    Uri(String),
}

fn to_facet(capture_group: Match, feature_mode: FeatureMode) -> Facet {
    let start = capture_group.start() as u16;
    let end = capture_group.end() as u16;
    let match_str = capture_group.as_str();
    Facet {
        index: to_facet_index(&start, &end),
        features: handle_facet_feature(feature_mode, match_str),
    }
}

fn handle_facet_feature(feature_mode: FeatureMode, match_str: &str) -> Vec<FacetFeatures> {
    match feature_mode {
        FeatureMode::Tag(_) => tags_to_facet_features(match_str),
        FeatureMode::Uri(_) => links_to_facet_features(match_str),
    }
}

fn to_facet_index(start: &u16, end: &u16) -> FacetIndex {
    FacetIndex {
        byte_start: *start,
        byte_end: *end,
    }
}

mod website_card_embeds {
    use crate::bluesky::create_header;
    use crate::ogp::Ogp;
    use crate::{ogp_scraping, set_headers, AccessToken};
    use curl::easy::{Easy};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::Path;
    use url::Url;

    pub fn create(access_token: &AccessToken, ogp: &Ogp) -> Embed {
        let dest = ".";
        ogp_scraping::fetch_image_by_ogp(ogp, dest);
        let ogp_image_path = format!("{}/{}", dest, ogp.get_image_name());
        let ogp_image_blob = upload_image_blob(
            access_token,
            ogp_image_path.as_str()
        );
        let thumb = Thumb::create(ogp, ogp_image_blob);
        let external = External::create(ogp, thumb);
        Embed::create(external)
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EmbedRoot {
        embed: Embed
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Embed {
        #[serde(rename = "$type")]
        _type: String,
        external: External,
    }

    impl Embed {
        fn create(external: External) -> Embed {
            Embed {
                _type: "app.bsky.embed.external".to_string(),
                external,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct External {
        uri: String,
        thumb: Thumb,
        title: String,
        description: String,
    }

    impl External {
        fn create(ogp: &Ogp, thumbnail: Thumb) -> External {
            let uri = &ogp.url;
            let title = &ogp.title;
            let desc = &ogp.desc;
            External {
                uri: uri.to_string(),
                thumb: thumbnail,
                title: title.to_string(),
                description: desc.to_string(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Thumb {
        #[serde(rename = "$type")]
        _type: String,
        #[serde(rename = "ref")]
        r#ref: Ref,
        #[serde(rename = "mimeType")]
        mime_type: String,
        size: u64,
    }

    impl Thumb {
        fn create(ogp: &Ogp, blob: UploadImageBlobToResponse) -> Thumb {
            let extension = ogp.get_image_extension();
            let image_type = extension_to_image_type(extension.as_str());
            let mime_type = get_mime_type(image_type);
            let file_name = ogp.get_image_name();
            let file_size = get_file_size(format!("./{}", file_name).as_str());
            let ref_data = blob.r#ref;

            Thumb {
                _type: "blob".to_string(),
                r#ref: ref_data,
                mime_type: mime_type.to_string(),
                size: file_size,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Ref {
        #[serde(rename = "$link")]
        _link: String,
    }

    impl Ref {
        fn get_link(&self) -> &str {
            &self._link
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UploadImageBlobToResponse {
        #[serde(rename = "$type")]
        _type: String,
        #[serde(rename = "ref")]
        r#ref: Ref,
        #[serde(rename = "mimeType")]
        mime_type: String,
        size: u64,
    }

    impl UploadImageBlobToResponse {
        fn create(blob: &serde_json::Value) -> UploadImageBlobToResponse {
            UploadImageBlobToResponse {
                _type: blob["$type"].to_string().replace('\"', ""),
                r#ref: Ref {
                    _link: blob["ref"]["$link"].to_string().replace('\"', ""),
                },
                mime_type: blob["mimeType"].to_string().replace('\"', ""),
                size: blob["size"].as_u64().unwrap(),
            }
        }
    }

    pub fn upload_image_blob(access_token: &AccessToken, file_path: &str) -> UploadImageBlobToResponse {
        let data = fs::read(file_path).expect("Failed to read the file.");

        if data.len() > 1_000_000 {
            panic!("Image file size too large. 1,000,000 bytes maximum, got:{}", data.len())
        }

        let mut response_data = Vec::new();
        let mut curl = Easy::new();
        let endpoint = "https://bsky.social/xrpc/com.atproto.repo.uploadBlob";
        curl.url(endpoint).unwrap();
        let content_type = "image/png";
        let headers = create_header(&access_token, content_type);
        let mut header_list = set_headers(headers);
        header_list.append("Accept: application/json").unwrap();
        curl.http_headers(header_list).unwrap();
        curl.post(true).unwrap();

        curl.post_fields_copy(&data).unwrap();
        {
            let mut transfer = curl.transfer();
            transfer
                .write_function(|data| {
                    response_data.extend_from_slice(data);
                    Ok(data.len())
                }).unwrap();
            transfer.perform().unwrap();
        }

        let res_string = String::from_utf8(response_data).unwrap();
        let sliced_res = res_string.as_str();
        let res_json: serde_json::Value = serde_json::from_str(sliced_res).unwrap();
        let blob = &res_json["blob"];

        UploadImageBlobToResponse::create(blob)
    }


    fn get_file_size(file_path: &str) -> u64 {
        fs::metadata(file_path).unwrap().len()
    }

    enum ImageType {
        JPEG,
        PNG,
        GIF,
        WebP,
        SVG,
        Unknown,
    }

    fn get_mime_type(image_type: ImageType) -> &'static str {
        match image_type {
            ImageType::JPEG => "image/jpeg",
            ImageType::PNG => "image/png",
            ImageType::GIF => "image/gif",
            ImageType::WebP => "image/webp",
            ImageType::SVG => "image/svg+xml",
            ImageType::Unknown => "",
        }
    }

    fn get_extension(url: &Url) -> String {
        let extension = Path::new(url.as_str()).extension().unwrap();
        extension.to_string_lossy().to_string()
    }

    fn extension_to_image_type(extension: &str) -> ImageType {
        match extension.to_lowercase().as_str() {
            "jpg" | "jpeg" => ImageType::JPEG,
            "png" => ImageType::PNG,
            "gif" => ImageType::GIF,
            "webp" => ImageType::WebP,
            "svg" => ImageType::SVG,
            _ => ImageType::Unknown,
        }
    }

    fn get_file_name(url: &Url) -> String {
        let file_name = Path::new(url.as_str()).file_name().unwrap();
        file_name.to_string_lossy().to_string()
    }

    fn get_ref(blob: Vec<u8>) -> Ref {
        let link = String::from_utf8(blob).unwrap();
        Ref {
            _link: link
        }
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

fn to_embed(capture_group: Match) {
    let url = capture_group.as_str();
    let ogps = ogp::get(url.to_string()).expect(&format!("Error occurred: Error"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bluesky::website_card_embeds::upload_image_blob;
    use crate::Receivers;

    #[test]
    fn can_create_tags() {
        let index1 = FacetIndex {
            byte_start: 29,
            byte_end: 34,
        };
        let features1 = vec![{
            FacetFeatures {
                facet_type: String::from("app.bsky.richtext.facet#tag"),
                feature_mode: FeatureMode::Tag(String::from("test")),
            }
        }];
        let index2 = FacetIndex {
            byte_start: 35,
            byte_end: 41,
        };
        let features2 = vec![{
            FacetFeatures {
                facet_type: String::from("app.bsky.richtext.facet#tag"),
                feature_mode: FeatureMode::Tag(String::from("test2")),
            }
        }];
        let expected = vec![
            Facet {
                index: index1,
                features: features1,
            },
            Facet {
                index: index2,
                features: features2,
            },
        ];
        let sut = create_tags_facets("ハッシュ投稿テスト\n\n#test #test2");
        println!("{:?}", sut);
        assert_eq!(
            sut.first().unwrap().index.byte_start,
            expected.first().unwrap().index.byte_start
        );
        assert_eq!(
            sut.first().unwrap().index.byte_end,
            expected.first().unwrap().index.byte_end
        );
        assert_eq!(
            sut.first().unwrap().features.first().unwrap().feature_mode,
            expected
                .first()
                .unwrap()
                .features
                .first()
                .unwrap()
                .feature_mode
        );
        assert_eq!(
            sut.get(1).unwrap().index.byte_start,
            expected.get(1).unwrap().index.byte_start
        );
        assert_eq!(
            sut.get(1).unwrap().index.byte_end,
            expected.get(1).unwrap().index.byte_end
        );
        assert_eq!(
            sut.get(1).unwrap().features.first().unwrap().feature_mode,
            expected
                .get(1)
                .unwrap()
                .features
                .first()
                .unwrap()
                .feature_mode
        );
    }

    #[test]
    fn can_find_hash_tags() {
        let hashes = [
            ["#test", "29", "34"],
            ["#日本語のテスト", "35", "57"],
            ["#1111", "58", "63"],
        ];

        let matches = find_hash_tags("ハッシュ投稿テスト\n\n#test #日本語のテスト #1111");
        for (hash_index, caps) in matches.iter().enumerate() {
            if let Some(captures) = caps.get(2) {
                println!("{}", hash_index);
                match hashes.get(hash_index) {
                    Some(hash) => {
                        assert_eq!(&captures.as_str(), hash.first().unwrap());
                        assert_eq!(
                            captures.start().to_string(),
                            hash.get(1).unwrap().to_string()
                        );
                        assert_eq!(captures.end().to_string(), hash.get(2).unwrap().to_string());
                    }
                    None => panic!("No hash found at index 0"),
                }
            }
        }
    }

    #[test]
    fn learn_find_hash_tags() {
        let text = "ハッシュ投稿テスト\n\n#test #test2 #3test #1111 #日本語のカタカナ。";
        // let pattern = r"(^|\s)(#[^\d\s]\w*)";
        // let regex_pattern = Regex::new(pattern).unwrap();
        //
        // let matches = regex_pattern.find_iter(text);
        let matches = find_hash_tags(text);
        println!("matches: {:?}", matches);
        for caps in matches {
            if let Some(cap) = caps.get(2) {
                println!("Matched: {}", cap.as_str());
                println!("Start: {}", cap.start());
                println!("End: {}", cap.end());
            }
        }
    }

    #[test]
    fn learn_find_link_strings() {
        let text =
            "Link test\n\n#hash #test\n\nhttps://www.example.com/url/?query=test&query2=test2";
        let matches = find_link_string(text);
        println!("matches: {:?}", matches);
        for caps in matches {
            if let Some(cap) = caps.get(2) {
                println!("Matched: {}", cap.as_str());
                println!("Start: {}", cap.start());
                println!("End: {}", cap.end());
            }
        }
    }

    #[test]
    fn learn_bluesky_get_profile() {
        let access_token = login();

        // send message
        match access_token {
            Ok(token) => {
                get_profile(&token);
            }
            Err(err) => {
                println!("Login failed.: {:?}", err)
            }
        }
    }

    #[test]
    fn learn_set_message() {
        let access_token = login().unwrap();
        let post_message = set_post_message(&access_token);
        print!("{:?}", post_message);
    }

    #[test]
    fn learn_upload_image_blob() {
        let access_token = login().unwrap();
        let message = upload_image_blob(&access_token, "./150x150.png");
        print!("{:?}", message);
    }

    #[test]
    fn learn_bluesky_send_message() {
        let access_token = login().unwrap();
        let result = send_message(&access_token);
        print!("{:?}", result);
    }

    #[test]
    fn learn_value() {
        let data = read_json_file("./tests/resources/message.json").unwrap();
        println!("{}", data.sender)

        // if let Some(sender) = data.get("sender") {
        //     println!("Sender -> {}", sender)
        // } else {
        //     println!("Sender not found")
        // }
    }

    #[test]
    fn learn_env() {
        let _ = dotenvy::dotenv();

        // for (key, value) in env::vars() {
        //     println!("{key}: {value}");
        // }
        match env::var("BLUESKY_APP_PASSWORD") {
            Ok(val) => println!("{val:?}"),
            Err(e) => println!("err: {e}"),
        }
    }

    #[test]
    fn can_read_json_file() {
        let result = read_json_file("./tests/resources/message.json").unwrap();
        assert_eq!(result.content, "Test message");
        assert_eq!(result.sender, "user1");
        assert_eq!(result.receivers.len(), 1);
        assert!(result.receivers.contains(&Receivers::BlueSky));
        assert_eq!(result.fixed_hashtags.mastodon, "#mastodon");
        assert_eq!(result.fixed_hashtags.bluesky, "#bluesky");
    }
}
