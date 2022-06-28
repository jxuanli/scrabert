use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::error::Error;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut set: HashSet<String> = HashSet::new();
    let urls =
        get_urls("https://stackoverflow.com/questions/16494822/why-is-it-called-rust").await?;
    for url in urls {
        set.insert(url);
    }
    for url in set.iter() {
        println!("{}", url);
    }
    Ok(())
}

async fn get_html(url: &str) -> Result<Document, Box<dyn Error>> {
    let html = reqwest::get(url).await?.text().await?;
    Ok(Document::from_read(Cursor::new(html))?)
}

async fn get_urls(starting_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut slices: Vec<&str> = starting_url.split("https://").collect();
    slices = slices[1].split("/").collect();
    let domain = slices[0];
    if domain.len() == 0 {
        return Ok(Vec::new());
    }
    let html = get_html(starting_url).await?;
    let links: Vec<String> = html
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|x| (*x).starts_with("https://"))
        .map(str::to_string)
        .filter(|x| {
            let mut tmp: Vec<&str> = (*x).split("https://").collect();
            tmp = tmp[1].split("/").collect();
            if tmp.len() == 1 || tmp[0].len() == 0 || tmp[1].len() == 0 {
                return false;
            }
            return tmp[0] == domain;
        })
        .collect();
    Ok(links)
}
