pub struct ImagePolicy {
    pub max_size_bytes: u64,
    pub max_width: u32,
    pub supported_formats: Vec<ImageFormat>,
}

pub enum ImageFormat {
    Jpeg,
    Png,
    Webp,
}

impl ImagePolicy {
    pub fn new(max_size_bytes: u64, max_width: u32, supported_formats: Vec<ImageFormat>) -> Self {
        Self {
            max_size_bytes,
            max_width,
            supported_formats,
        }
    }

    pub fn social_media_default() -> Self {
        const KB: u64 = 1024;
        Self {
            max_size_bytes: KB * 100, // 100KB
            max_width: 640,
            supported_formats: vec![ImageFormat::Webp],
        }
    }

    pub fn validate_size(&self, image_size_bytes: u64) -> bool {
        image_size_bytes <= self.max_size_bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::domain::image::model::image_policy::{ImageFormat, ImagePolicy};

    #[test]
    fn test_social_media_default() {
        let policy = ImagePolicy::social_media_default();
        assert_eq!(policy.max_size_bytes, 102400);
        assert_eq!(policy.max_width, 640);
        assert!(matches!(policy.supported_formats[0], ImageFormat::Webp));
    }

    #[test]
    fn test_validate_size() {
        let policy = ImagePolicy::social_media_default();
        assert!(policy.validate_size(102400));
        assert!(!policy.validate_size(102401));
    }
}
