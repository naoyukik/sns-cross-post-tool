use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadedImageBlob {
    #[serde(rename = "$type")]
    _type: String,
    #[serde(rename = "ref")]
    r#ref: UploadedImageBlobRef,
    #[serde(rename = "mimeType")]
    mime_type: String,
    size: u64,
}

impl UploadedImageBlob {
    pub fn new(blob: &serde_json::Value) -> UploadedImageBlob {
        UploadedImageBlob {
            _type: blob["$type"].to_string().replace('\"', ""),
            r#ref: UploadedImageBlobRef {
                _link: blob["ref"]["$link"].to_string().replace('\"', ""),
            },
            mime_type: blob["mimeType"].to_string().replace('\"', ""),
            size: blob["size"].as_u64().unwrap(),
        }
    }

    pub fn get_type(&self) -> String {
        self._type.to_string()
    }

    pub fn get_ref(&self) -> UploadedImageBlobRef {
        self.r#ref.clone()
    }

    pub fn get_mime_type(&self) -> String {
        self.mime_type.to_string()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadedImageBlobRef {
    #[serde(rename = "$link")]
    _link: String,
}

impl UploadedImageBlobRef {
    pub fn get_link(&self) -> &str {
        &self._link
    }
}
