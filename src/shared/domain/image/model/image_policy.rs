pub struct ImagePolicy {
    pub max_size_bytes: u64,
    pub supported_formats: Vec<ImageFormat>,
}

pub enum ImageFormat {
    Jpeg,
    Png,
    Webp,
}

impl ImagePolicy {
    pub fn max_size_bytes() -> u64 {
        const KB: u64 = 1024;
        KB * 100 // 100KB
    }

    pub fn validate_size(&self, image_size_bytes: u64) -> bool {
        image_size_bytes <= self.max_size_bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::domain::image::model::image_policy::{ImageFormat, ImagePolicy};

    #[test]
    fn test_max_size_bytes() {
        let policy = ImagePolicy {
            max_size_bytes: ImagePolicy::max_size_bytes(),
            supported_formats: vec![ImageFormat::Jpeg],
        };
        assert_eq!(policy.max_size_bytes, 102400);
    }

    #[test]
    fn test_validate_size() {
        let policy = ImagePolicy {
            max_size_bytes: ImagePolicy::max_size_bytes(),
            supported_formats: vec![ImageFormat::Jpeg],
        };
        assert!(policy.validate_size(102400));
    }
}
