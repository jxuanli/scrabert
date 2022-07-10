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

    async fn predict(s: mpsc::SyncSender<Message>, texts: Vec<String>) -> Result<String> {
        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| s.send((texts, sender)))?;
        Ok(receiver.await?.iter().fold("".to_owned(), |acc, x| format!("{}{}", acc, x)).to_owned())
    }

    async fn respond(contents: Vec<String>) -> Result<String> {
        let mut sp = Spinner::new(Spinners::Dots6, "\t\t I am thinking!");
        sp.start();
        let tmp = Self::handler(contents).await?;
        sp.stop();
        Ok(tmp)
    }

    async fn handler(contents: Vec<String>) -> Result<String>;
}

pub async fn talk(input: &str) -> Result<String> {
    let mut res = Ok(String::new());
    let input_lower_case = input.trim().to_lowercase();
    let tmp = input_lower_case.clone();
    let question_indicators = vec!["what", "which", "when", "where", "who", "whom", "whose", "why", "whether", "how"];
    let is_question = question_indicators.iter().fold(false, |acc, x| tmp.starts_with(x) || acc) || tmp.ends_with("?"); 
    if is_question {
        if !input_lower_case.ends_with("?") {
            res = get_answer(&format!("{}{}", input_lower_case, "?")).await;
        } else {
            res = get_answer(input_lower_case.as_str()).await;
        }
    } else {
        res = get_response(input_lower_case.as_str()).await;
    }
    res
}

async fn get_summary(request: &str) -> Result<String> {
    let contents = executor::block_on(scraper::scrape(request)).unwrap();
    summarizer::Summarizer::respond(contents[0].clone()).await
}

async fn get_answer(question: &str) -> Result<String> {
    let contexts = get_summary(question).await?;
    let str_vec = vec![
        question.to_owned(),
        contexts.to_owned(),
    ];
    qa::QuestionAnswerer::respond(str_vec).await
}

async fn get_response(input: &str) -> Result<String> {
    let contents = vec![ 
        input.to_owned(),
    ];
    conversation::Communicator::respond(contents).await
}
