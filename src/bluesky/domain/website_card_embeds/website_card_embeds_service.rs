use crate::bluesky::domain::login::model::access_token::AccessToken;
use crate::bluesky::domain::website_card_embeds::model::embed::Embed;
use crate::bluesky::domain::website_card_embeds::model::uploaded_image_blob::UploadedImageBlob;
use crate::bluesky::util::http::create_header;
use crate::ogp::Ogp;
use crate::ogp_scraping;
use crate::util::set_headers;
use curl::easy::Easy;
use std::fs;
use std::path::Path;
use url::Url;

pub fn create_website_card_embeds(access_token: &AccessToken, ogp: &Ogp) -> Embed {
    let dest = ".";
    ogp_scraping::fetch_image_by_ogp(ogp, dest);
    let ogp_image_path = format!("{}/{}", dest, ogp.get_image_name());
    let uploaded_image_blob = upload_image_blob(access_token, ogp_image_path.as_str());
    Embed::new(ogp, &uploaded_image_blob)
}

fn upload_image_blob(access_token: &AccessToken, file_path: &str) -> UploadedImageBlob {
    let data = fs::read(file_path).expect("Failed to read the file.");

    if data.len() > 1_000_000 {
        panic!(
            "Image file size too large. 1,000,000 bytes maximum, got:{}",
            data.len()
        )
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
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    let res_string = String::from_utf8(response_data).unwrap();
    let sliced_res = res_string.as_str();
    let res_json: serde_json::Value = serde_json::from_str(sliced_res).unwrap();
    let blob = &res_json["blob"];

    UploadedImageBlob::new(blob)
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
