use crate::shared::domain::image::model::image_policy::{ImageFormat, ImagePolicy};
use crate::shared::domain::image_repository::ImageRepository;
use librusimg::{Extension, RusImg, RusimgError, SaveStatus};
use std::path::Path;

pub struct ImageRepositoryImpl {}

impl ImageRepository for ImageRepositoryImpl {
    fn compress_and_save_for_social_media(file_path: &str) -> Result<SaveStatus, RusimgError> {
        let policy = ImagePolicy::social_media_default();

        let mut quality = 100.0;
        let min_quality = 10.0;
        let step = 5.0;

        // ファイルをpolicyに従って圧縮する
        loop {
            // 画質劣化の累積を防ぐため、ループのたびにファイルを読み込み直す
            let mut rusimg = Self::file_open(file_path)?;

            Self::compress_for_social_media(&mut rusimg, quality, &policy)?;
            let save_status = Self::save(&mut rusimg)?;

            // 保存されたファイルのafter_filesizeを使ってポリシーを満たすか検証
            let is_valid = save_status
                .after_filesize
                .map(|size| policy.validate_size(size))
                .unwrap_or(false);

            if is_valid || quality <= min_quality {
                return Ok(save_status);
            }

            quality -= step;
        }
    }
}

impl ImageRepositoryImpl {
    fn file_open(file_path: &str) -> Result<RusImg, RusimgError> {
        let path = Path::new(file_path);
        RusImg::open(path)
    }

    fn compress_for_social_media(
        rusimg: &mut RusImg,
        quality: f32,
        policy: &ImagePolicy,
    ) -> Result<(), RusimgError> {
        // ポリシーに基づいてフォーマットを選択（現在はWebP固定）
        // 将来的にはpolicy.supported_formatsから選択するロジックが必要かもしれない
        if policy
            .supported_formats
            .iter()
            .any(|f| matches!(f, ImageFormat::Webp))
        {
            rusimg.convert(&Extension::Webp)?;
        }

        let image_size = rusimg.get_image_size()?;
        let max_width = policy.max_width as usize;

        if image_size.width > max_width {
            rusimg.resize(max_width as f32 / image_size.width as f32 * 100.0)?;
        }
        rusimg.compress(Some(quality))?;
        Ok(())
    }

    fn save(rusimg: &mut RusImg) -> Result<SaveStatus, RusimgError> {
        let save_status = rusimg.save_image(Some("./storage/downloaded_images/compressed"))?;
        Ok(save_status)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::domain::image::model::image_policy::ImagePolicy;
    use crate::shared::domain::image_repository::ImageRepository;
    use crate::shared::infrastructure::image_repository_impl::ImageRepositoryImpl;

    #[test]
    fn test_compress_and_save_for_social_media() {
        let compressed = ImageRepositoryImpl::compress_and_save_for_social_media(
            "./tests/resources/shared/infrastructure/Gemini_Generated_Image_compression_test.jpg",
        )
        .unwrap();
        assert!(compressed.after_filesize <= Some(100 * 1024));
    }

    #[test]
    fn test_file_open() {
        let opened_file = ImageRepositoryImpl::file_open(
            "./tests/resources/shared/infrastructure/Gemini_Generated_Image_compression_test.jpg",
        );
        assert!(opened_file.is_ok());
    }

    #[test]
    fn test_compress_for_social_media() {
        let opened_file = ImageRepositoryImpl::file_open(
            "./tests/resources/shared/infrastructure/Gemini_Generated_Image_compression_test.jpg",
        );
        let mut rusimg = opened_file.unwrap();
        let quality = 100.0;
        let policy = ImagePolicy::social_media_default();
        let compressed_rusimg =
            ImageRepositoryImpl::compress_for_social_media(&mut rusimg, quality, &policy);
        assert!(compressed_rusimg.is_ok());
    }

    #[test]
    fn test_save() {
        let opened_file = ImageRepositoryImpl::file_open(
            "./tests/resources/shared/infrastructure/Gemini_Generated_Image_compression_test.jpg",
        );
        let mut rusimg = opened_file.unwrap();
        let quality = 100.0;
        let policy = ImagePolicy::social_media_default();
        let _ = ImageRepositoryImpl::compress_for_social_media(&mut rusimg, quality, &policy);
        let saved_rusimg = ImageRepositoryImpl::save(&mut rusimg);
        assert!(saved_rusimg.is_ok());
    }
}
