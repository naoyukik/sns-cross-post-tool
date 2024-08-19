use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

fn get_html(url: &str) -> Result<Vec<u8>, curl::Error> {
    let mut easy = curl::easy::Easy::new();
    easy.url(&url)?;
    let mut html = Vec::new();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            html.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    Ok(html)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ogp {
    pub title: String,
    pub desc: String,
    pub image: String,
    pub url: String,
}

fn extract(html: Vec<u8>) -> Ogp {
    let html_str = String::from_utf8(html).expect("Failed to convert Vec<u8> to String");
    let fragment = Html::parse_fragment(&html_str);
    let title_selector = Selector::parse("meta[property='og:title']").unwrap();
    let description_selector = Selector::parse("meta[property='og:description']").unwrap();
    let image_selector = Selector::parse("meta[property='og:image']").unwrap();
    let url_selector = Selector::parse("meta[property='og:url']").unwrap();
    Ogp {
        title: fragment.select(&title_selector).next().map(|element| element.value().attr("content").unwrap_or("")).unwrap_or("").to_string(),
        desc: fragment.select(&description_selector).next().map(|element| element.value().attr("content").unwrap_or("")).unwrap_or("").to_string(),
        image: fragment.select(&image_selector).next().map(|element| element.value().attr("content").unwrap_or("")).unwrap_or("").to_string(),
        url: fragment.select(&url_selector).next().map(|element| element.value().attr("content").unwrap_or("")).unwrap_or("").to_string(),
    }
}

pub fn get(url: String) -> Result<Ogp, String> {
    let ogp = match get_html(&url) {
        Ok(html) => extract(html),
        Err(err) => return Err(format!("Error: {}", err))
    };
    Ok(ogp)
}

#[cfg(test)]
mod tests {
    use crate::ogp::{extract, get_html};

    // #[test]
    // fn can_scrape_html() {
    //     let url = "https://...".to_string();
    //     let ogp = scrape_ogp(url);
    // }

    // #[test]
    // fn can_get_html() {
    //     let url = "https://...";
    //     let html = get_html(url).unwrap();
    //     let string = String::from_utf8(html).unwrap();
    //     println!("{}", string)
    // }

    #[test]
    fn can_extract_ogp() {
        let html = r#"<html><head>
                <meta property="og:title" content="Example Page Title">
                <meta property="og:description" content="Description of the page.">
                <meta property="og:image" content="https://example.com/sample.jpg">
                <meta property="og:url" content="https://example.com/">
            </head></html>"#.as_bytes().to_vec();

        let ogp = extract(html);
        assert_eq!(ogp.title, "Example Page Title");
        assert_eq!(ogp.desc, "Description of the page.");
        assert_eq!(ogp.image, "https://example.com/sample.jpg");
        assert_eq!(ogp.url, "https://example.com/");
    }

    #[test]
    fn empty_extract_ogp() {
        let html = r#"<html><head>
            </head></html>"#.as_bytes().to_vec();

        let ogp = extract(html);
        assert_eq!(ogp.title, "");
        assert_eq!(ogp.desc, "");
        assert_eq!(ogp.image, "");
        assert_eq!(ogp.url, "");
    }
}
