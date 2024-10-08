use crate::bluesky::domain::website_card_embeds::website_card_embeds_service::UploadedImageBlobDto;
use crate::ogp::Ogp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedDto {
    #[serde(rename = "$type")]
    _type: String,
    external: External,
}

impl EmbedDto {
    pub fn create(ogp: &Ogp, uploaded_image_blob: &UploadedImageBlobDto) -> EmbedDto {
        let thumb = Thumb::from_uploaded_image_blob(uploaded_image_blob);
        let external = External::from_ogp_and_thumb(ogp, thumb);
        EmbedDto {
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
    fn from_ogp_and_thumb(ogp: &Ogp, thumbnail: Thumb) -> External {
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
    fn from_uploaded_image_blob(blob: &UploadedImageBlobDto) -> Thumb {
        // let extension = ogp.get_image_extension();
        // let image_type = crate::bluesky::domain::website_card_embeds::website_card_embeds::extension_to_image_type(extension.as_str());
        // let mime_type = crate::bluesky::domain::website_card_embeds::website_card_embeds::get_mime_type(image_type);
        // let file_name = ogp.get_image_name();
        let mime_type = blob.get_mime_type();
        let file_size = blob.get_size();
        // let file_size = crate::bluesky::domain::website_card_embeds::website_card_embeds::get_file_size(format!("./{}", file_name).as_str());
        let ref_data = blob.get_ref();

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
