use curl::easy::List;

pub trait HttpService {
    fn set_headers(header_list: Vec<String>) -> List;
}

pub struct HttpServiceImpl {}

impl HttpService for HttpServiceImpl {
    fn set_headers(header_list: Vec<String>) -> List {
        let mut headers = List::new();
        for header in header_list {
            headers.append(header.as_str()).unwrap();
        }
        headers
    }
}
