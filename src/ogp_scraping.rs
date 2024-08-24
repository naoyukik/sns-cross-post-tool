use std::fs::File;
use std::io::Write;
use curl::easy::Easy;
use crate::ogp::Ogp;


pub fn fetch_image_by_ogp(ogp: Ogp, dest: &str) {
    let mut response_data = Vec::new();
    let mut curl = Easy::new();
    let endpoint = ogp.image;
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

    let mut file = match File::create(dest) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", dest, why)
    };
    match file.write_all(&response_data) {
        Ok(_) => println!("Successfully wrote to {}", dest),
        Err(why) => panic!("couldn't write to {}:{}", dest, why)
    }
}
