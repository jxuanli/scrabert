#![forbid(unsafe_code)]

mod scraper;
mod summarizer;

use anyhow::Result;
use futures::executor;
use spinners_rs::{Spinner, Spinners};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<()> {
    let contents = executor::block_on(scraper::scrape()).unwrap();
    let mut sp = Spinner::new(Spinners::Dots6, "\t\t I am thinking!");
    sp.start();
    let mut tmp: HashSet<Vec<String>> = HashSet::new();
    for content in contents {
        let (_handle, classifier) = summarizer::Summarizer::spawn();
        let summarization = classifier.predict(content).await?;
        tmp.insert(summarization);
    }
    sp.stop();
    let summarizations: Vec<String> = tmp.iter().filter_map(|x| Some((*x)[0].replace("[X_SEP]", "").replace("  ", " "))).collect();
    for s in summarizations {
        println!("{:?}", s);
    }
    Ok(())
}
