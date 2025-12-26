use librusimg::{RusimgError, SaveStatus};

pub trait ImageRepository {
    fn compress_and_save_for_social_media(file_path: &str) -> Result<SaveStatus, RusimgError>;
}
