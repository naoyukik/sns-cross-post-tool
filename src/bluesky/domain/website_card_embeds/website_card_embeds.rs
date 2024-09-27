use crate::ogp::Ogp;
use crate::{ogp_scraping, set_headers, AccessToken};
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use url::Url;

pub fn create(access_token: &AccessToken, ogp: &Ogp) -> Embed {
    let dest = ".";
    ogp_scraping::fetch_image_by_ogp(ogp, dest);
    let ogp_image_path = format!("{}/{}", dest, ogp.get_image_name());
    let ogp_image_blob = upload_image_blob(access_token, ogp_image_path.as_str());
    let thumb = Thumb::create(ogp, ogp_image_blob);
    let external = External::create(ogp, thumb);
    Embed::create(external)
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct EmbedRoot {
//     embed: Embed,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    #[serde(rename = "$type")]
    _type: String,
    external: External,
}

impl Embed {
    pub fn create(external: External) -> Embed {
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

pub fn upload_image_blob(
    access_token: &AccessToken,
    file_path: &str,
) -> UploadImageBlobToResponse {
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
    Ref { _link: link }
}
