#![forbid(unsafe_code)]

mod scraper;
mod summarizer;

use anyhow::Result;
use futures::executor;
use spinners_rs::{Spinner, Spinners};

#[tokio::main]
async fn main() -> Result<()> {
    let tmp = executor::block_on(scraper::scrape()).unwrap();
    let mut sp = Spinner::new(Spinners::Dots9, "\t\t I am thinking!");
    sp.start();
    let (_handle, classifier) = summarizer::Summarizer::spawn();
    let summarization = classifier.predict(tmp).await?;
    sp.stop();
    println!("\n Results: {:?}", summarization);
    Ok(())
}
