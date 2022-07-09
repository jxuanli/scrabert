#![forbid(unsafe_code)]
mod conversation;
mod qa;
mod scraper;
mod summarizer;
use anyhow::Result;
use async_trait::async_trait;
use futures::executor;
use spinners_rs::{Spinner, Spinners};
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tokio::{sync::oneshot, task};

pub type Message = (Vec<String>, oneshot::Sender<Vec<String>>);

#[async_trait]
pub trait Bert {
    fn spawn() -> (JoinHandle<Result<()>>, mpsc::SyncSender<Message>) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));
        (handle, sender)
    }

    fn runner(receiver: mpsc::Receiver<Message>) -> Result<()>;

    async fn predict(s: mpsc::SyncSender<Message>, texts: Vec<String>) -> Result<Vec<String>> {
        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| s.send((texts, sender)))?;
        Ok(receiver.await?)
    }

    async fn respond(contents: Vec<Vec<String>>) -> Result<Vec<String>> {
        let mut sp = Spinner::new(Spinners::Dots6, "\t\t I am thinking!");
        sp.start();
        let tmp = Self::handler(contents).await?;
        sp.stop();
        Ok(tmp)
    }

    async fn handler(contents: Vec<Vec<String>>) -> Result<Vec<String>>;
}

async fn get_summary(request: &str) -> Result<String> {
    let contents = executor::block_on(scraper::scrape(request)).unwrap();
    let tmp = summarizer::Summarizer::respond(contents).await?;
    Ok(tmp.iter().fold("".to_owned(), |acc, x| acc.clone() + x))
}

pub async fn get_answer(question: &str) -> Result<String> {
    let contexts = get_summary(question).await?;
    let mut tmp = Vec::new();
    tmp.push(question.to_owned());
    tmp.push(contexts.to_owned());
    let mut str_vec = Vec::new();
    str_vec.push(tmp);
    tmp = qa::QuestionAnswerer::respond(str_vec).await?;
    Ok(tmp.iter().fold("".to_owned(), |acc, x| acc.clone() + x))
}

pub async fn get_response(contents: Vec<Vec<String>>) -> Result<String> {
    let tmp = conversation::Communicator::respond(contents).await?;
    Ok(tmp.iter().fold("".to_owned(), |acc, x| acc.clone() + x))
}
