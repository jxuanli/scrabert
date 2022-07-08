#![forbid(unsafe_code)]

use anyhow::Result;
use async_trait::async_trait;
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
}
