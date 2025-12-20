use crate::shared::domain::image_repository::ImageRepository;
use librusimg::{Extension, RusImg, RusimgError, SaveStatus};
use std::path::Path;

pub struct ImageRepositoryImpl {}

impl ImageRepository for ImageRepositoryImpl {
    fn compress_and_save_for_social_media(file_path: &str) -> Result<SaveStatus, RusimgError> {
        let mut rusimg = Self::file_open(file_path)?;

        Self::compress_for_social_media(&mut rusimg)?;
        Self::save(&mut rusimg)
    }
}

impl ImageRepositoryImpl {
    fn file_open(file_path: &str) -> Result<RusImg, RusimgError> {
        let path = Path::new(file_path);
        RusImg::open(path)
    }

    fn compress_for_social_media(rusimg: &mut RusImg) -> Result<(), RusimgError> {
        rusimg.convert(&Extension::Webp)?;
        let image_size = rusimg.get_image_size()?;
        if image_size.width > 1024 {
            rusimg.resize(1024.0 / image_size.width as f32 * 100.0)?;
        }
        rusimg.compress(Some(90.0))?;
        Ok(())
    }

    fn save(rusimg: &mut RusImg) -> Result<SaveStatus, RusimgError> {
        let save_status = rusimg.save_image(Some("./storage/downloaded_images/compressed"))?;
        Ok(save_status)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::infrastructure::image_repository_impl::ImageRepositoryImpl;

    #[test]
    fn test_file_open() {
        let opened_file =
            ImageRepositoryImpl::file_open("./storage/downloaded_images/14992376389114161441");
        assert!(opened_file.is_ok());
    }

    #[test]
    fn test_compress_for_social_media() {
        let opened_file =
            ImageRepositoryImpl::file_open("./storage/downloaded_images/14992376389114161441");
        let mut rusimg = opened_file.unwrap();
        let compressed_rusimg = ImageRepositoryImpl::compress_for_social_media(&mut rusimg);
        assert!(compressed_rusimg.is_ok());
    }

    #[test]
    fn test_save() {
        let opened_file =
            ImageRepositoryImpl::file_open("./storage/downloaded_images/14992376389114161441");
        let mut rusimg = opened_file.unwrap();
        let extension = rusimg.get_extension();
        let compressed_rusimg = ImageRepositoryImpl::compress_for_social_media(&mut rusimg);
        let saved_rusimg = ImageRepositoryImpl::save(&mut rusimg);
        assert!(saved_rusimg.is_ok());
    }
}
