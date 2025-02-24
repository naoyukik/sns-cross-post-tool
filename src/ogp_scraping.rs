use crate::ogp;
use crate::ogp::Ogp;
use curl::easy::Easy;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;

pub fn fetch_image_by_ogp(ogp: &Ogp, dest: &str) {
    let mut response_data = Vec::new();
    let mut curl = Easy::new();
    let endpoint = &ogp.image;
    curl.url(endpoint.as_str()).unwrap();

    {
        let mut transfer = curl.transfer();
        transfer
            .write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    let image_name = &ogp.get_image_name();
    let temp_filename = create_temp_filename(image_name);
    let file_path = format!("{}/{}", dest, temp_filename);
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", dest, why),
    };
    match file.write_all(&response_data) {
        Ok(_) => info!("Successfully wrote to {}", dest),
        Err(why) => panic!("couldn't write to {}:{}", dest, why),
    }
}

pub fn fetch_ogp_data(url_string: String) -> Result<Ogp, curl::Error> {
    let ogp = ogp::get(url_string)?;
    Ok(ogp)
}

fn create_temp_filename(url: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    format!("{}", hasher.finish())
}
