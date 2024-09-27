use curl::easy::List;
use serde::{Deserialize, Serialize};

pub fn set_headers(header_list: Vec<String>) -> List {
    let mut headers = List::new();
    for header in header_list {
        headers.append(header.as_str()).unwrap();
    }
    headers
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
}
