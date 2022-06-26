use std::error::Error;
use std::io::Cursor;
use select::document::Document;
use select::predicate::{Name};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let html = get_html("https://stackoverflow.com/questions/16494822/why-is-it-called-rust").await?;
    let _links = html.find(Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .filter(|x| (*x).starts_with("https://"))
                    .for_each(|x| println!("{}", x));
    Ok(())
}

async fn get_html(url: &str) -> Result<Document, Box<dyn Error>> {
    let html = reqwest::get(url).await?.text().await?;
    Ok(Document::from_read(Cursor::new(html))?)
}