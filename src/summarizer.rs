#![forbid(unsafe_code)]
use anyhow::Result;
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
use rust_bert::resources::RemoteResource;
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tokio::{sync::oneshot, task};

type Message = (Vec<String>, oneshot::Sender<Vec<String>>);

#[derive(Debug, Clone)]
pub struct Summarizer {
    sender: mpsc::SyncSender<Message>,
}

impl Summarizer {
    pub fn spawn() -> (JoinHandle<Result<()>>, Summarizer) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));
        (handle, Summarizer { sender })
    }

    fn runner(receiver: mpsc::Receiver<Message>) -> Result<()> {
        let config_resource = RemoteResource::from_pretrained(T5ConfigResources::T5_SMALL);
        let vocab_resource = RemoteResource::from_pretrained(T5VocabResources::T5_SMALL);
        let weights_resource = RemoteResource::from_pretrained(T5ModelResources::T5_SMALL);

        let summarization_config = SummarizationConfig::new(
            ModelType::T5,
            weights_resource,
            config_resource,
            vocab_resource.clone(),
            vocab_resource,
        );
        let model = SummarizationModel::new(summarization_config)?;
        while let Ok((texts, sender)) = receiver.recv() {
            let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
            let summarization = model.summarize(&[texts[0]]);
            sender.send(summarization).expect("sending results");
        }

        Ok(())
    }

    pub async fn predict(&self, texts: Vec<String>) -> Result<Vec<String>> {
        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((texts, sender)))?;
        Ok(receiver.await?)
    }
}
