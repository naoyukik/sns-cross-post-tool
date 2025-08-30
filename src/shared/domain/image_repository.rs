use librusimg::{RusImg, RusimgError, SaveStatus};

pub trait ImageRepository {
    fn file_open(file_path: &str) -> Result<RusImg, RusimgError>;
    fn compress_for_social_media(rusimg: &mut RusImg) -> Result<(), RusimgError>;
    fn save(rusimg: &mut RusImg) -> Result<SaveStatus, RusimgError>;
}
