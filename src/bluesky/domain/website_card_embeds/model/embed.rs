use crate::bluesky::domain::website_card_embeds::model::uploaded_image_blob::UploadedImageBlob;
use crate::ogp::Ogp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    #[serde(rename = "$type")]
    _type: String,
    external: EmbedExternal,
}

impl Embed {
    pub fn new(ogp: &Ogp, uploaded_image_blob: &UploadedImageBlob) -> Embed {
        let thumb = EmbedExternalThumb::new(uploaded_image_blob);
        let external = EmbedExternal::new(ogp, thumb);
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
    fn new(ogp: &Ogp, thumbnail: EmbedExternalThumb) -> EmbedExternal {
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
    fn new(blob: &UploadedImageBlob) -> EmbedExternalThumb {
        let mime_type = blob.get_mime_type();
        let file_size = blob.get_size();
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
}
