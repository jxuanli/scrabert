#![forbid(unsafe_code)]

mod bert;
mod conversation;
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
    let mut tmp = Vec::new();
    tmp.push("Where is Amy?".to_owned());
    tmp.push("Amy is in Vancouver.".to_owned());
    let answer = get_answers(tmp).await?;
    println!("{:?}", answer);
    tmp = Vec::new();
    tmp.push("I like cats!".to_owned());
    let response = get_response(tmp).await?;
    println!("{:?}", response);
    Ok(())
}

async fn get_summaries(contents: Vec<Vec<String>>) -> Result<Vec<String>> {
    let mut sp = Spinner::new(Spinners::Dots6, "\t\t I am thinking!");
    sp.start();
    let mut tmp: HashSet<Vec<String>> = HashSet::new();
    for content in contents {
        let (_handle, sender) = summarizer::Summarizer::spawn();
        let summarization = summarizer::Summarizer::predict(sender, content).await?;
        tmp.insert(summarization);
    }
    sp.stop();
    let summarizations: Vec<String> = tmp
        .iter()
        .filter_map(|x| Some((*x)[0].replace("  ", " ")))
        .collect();
    Ok(summarizations)
}

async fn get_answers(contents: Vec<String>) -> Result<String> {
    let mut sp = Spinner::new(Spinners::Dots6, "\t\t I am thinking!");
    sp.start();
    let (_handle, sender) = qa::QuestionAnswerer::spawn();
    let qa_ins = qa::QuestionAnswerer::predict(sender, contents).await?;
    sp.stop();
    Ok(qa_ins[0].clone())
}

async fn get_response(contents: Vec<String>) -> Result<String> {
    let mut sp = Spinner::new(Spinners::Dots6, "\t\t I am thinking!");
    sp.start();
    let (_handle, sender) = conversation::Communicator::spawn();
    let response = conversation::Communicator::predict(sender, contents).await?;
    sp.stop();
    Ok(response[0].clone())
}
