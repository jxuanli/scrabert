#![forbid(unsafe_code)]

mod bert;
mod qa;
mod scraper;
mod summarizer;
use crate::bert::Bert;
use anyhow::Result;
use futures::executor;
use spinners_rs::{Spinner, Spinners};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<()> {
    let contents = executor::block_on(scraper::scrape()).unwrap();
    let summaries = get_summaries(contents).await?;
    for s in summaries {
        println!("{}", s);
    }
    let mut sp = Spinner::new(Spinners::Dots6, "\t\t I am thinking!");
    sp.start();
    let (_handle, classifier) = qa::QAer::spawn();
    let mut tmp = Vec::new();
    tmp.push("Where is Amy?".to_owned());
    tmp.push("Amy is in Vancouver.".to_owned());
    let qa_ins = qa::QAer::predict(classifier, tmp).await?;
    sp.stop();
    println!("{:?}", qa_ins);
    Ok(())
}

async fn get_summaries(contents: Vec<Vec<String>>) -> Result<Vec<String>> {
    let mut sp = Spinner::new(Spinners::Dots6, "\t\t I am thinking!");
    sp.start();
    let mut tmp: HashSet<Vec<String>> = HashSet::new();
    for content in contents {
        let (_handle, classifier) = summarizer::Summarizer::spawn();
        let summarization = summarizer::Summarizer::predict(classifier, content).await?;
        tmp.insert(summarization);
    }
    sp.stop();
    let summarizations: Vec<String> = tmp
        .iter()
        .filter_map(|x| Some((*x)[0].replace("  ", " ")))
        .collect();
    Ok(summarizations)
}
