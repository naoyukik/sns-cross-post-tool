use crate::bluesky::domain::website_card_embeds::website_card_embeds_service::UploadedImageBlobDto;
use crate::ogp::Ogp;
use serde::{Deserialize, Serialize};
use crate::bluesky::domain::website_card_embeds::model::uploaded_image_blob::UploadedImageBlob;

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    #[serde(rename = "$type")]
    _type: String,
    external: EmbedExternal,
}

impl Embed {
    pub fn create(ogp: &Ogp, uploaded_image_blob: &UploadedImageBlobDto) -> Embed {
        let thumb = EmbedExternalThumb::from_uploaded_image_blob(uploaded_image_blob);
        let external = EmbedExternal::from_ogp_and_thumb(ogp, thumb);
        Embed {
            _type: "app.bsky.embed.external".to_string(),
            external,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedExternal {
    uri: String,
    thumb: EmbedExternalThumb,
    title: String,
    description: String,
}

impl EmbedExternal {
    fn from_ogp_and_thumb(ogp: &Ogp, thumbnail: EmbedExternalThumb) -> EmbedExternal {
        let uri = &ogp.url;
        let title = &ogp.title;
        let desc = &ogp.desc;
        EmbedExternal {
            uri: uri.to_string(),
            thumb: thumbnail,
            title: title.to_string(),
            description: desc.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedExternalThumb {
    #[serde(rename = "$type")]
    _type: String,
    #[serde(rename = "ref")]
    r#ref: EmbedExternalThumbRef,
    #[serde(rename = "mimeType")]
    mime_type: String,
    size: u64,
}

impl EmbedExternalThumb {
    fn from_uploaded_image_blob(blob: &UploadedImageBlob) -> EmbedExternalThumb {
        // let extension = ogp.get_image_extension();
        // let image_type = crate::bluesky::domain::website_card_embeds::website_card_embeds::extension_to_image_type(extension.as_str());
        // let mime_type = crate::bluesky::domain::website_card_embeds::website_card_embeds::get_mime_type(image_type);
        // let file_name = ogp.get_image_name();
        let mime_type = blob.get_mime_type();
        let file_size = blob.get_size();
        // let file_size = crate::bluesky::domain::website_card_embeds::website_card_embeds::get_file_size(format!("./{}", file_name).as_str());
        let ref_data = blob.get_ref();

        EmbedExternalThumb {
            _type: "blob".to_string(),
            r#ref: EmbedExternalThumbRef::new(ref_data.get_link()),
            mime_type: mime_type.to_string(),
            size: file_size,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedExternalThumbRef {
    #[serde(rename = "$link")]
    _link: String,
}

impl EmbedExternalThumbRef {
    fn new(url: &str) -> EmbedExternalThumbRef {
        EmbedExternalThumbRef {
            _link: url.to_string(),
        }
    }

    fn get_link(&self) -> &str {
        &self._link
    }
}
