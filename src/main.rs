#![forbid(unsafe_code)]

mod scraper;
mod summarizer;

use anyhow::Result;
use futures::executor;

#[tokio::main]
async fn main() -> Result<()> {
    let tmp = executor::block_on(scraper::scrape()).unwrap();
    let (_handle, classifier) = summarizer::Summarizer::spawn();
    let summarization = classifier.predict(tmp).await?;
    println!("Results: {:?}", summarization);
    Ok(())
}
