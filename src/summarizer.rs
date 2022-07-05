#![forbid(unsafe_code)]
use anyhow::Result;
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::prophetnet::{
    ProphetNetConfigResources, ProphetNetModelResources, ProphetNetVocabResources,
};
use rust_bert::resources::RemoteResource;
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tch::Device;
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
        let config_resource = Box::new(RemoteResource::from_pretrained(
            ProphetNetConfigResources::PROPHETNET_LARGE_CNN_DM,
        ));
        let vocab_resource = Box::new(RemoteResource::from_pretrained(
            ProphetNetVocabResources::PROPHETNET_LARGE_CNN_DM,
        ));
        let weights_resource = Box::new(RemoteResource::from_pretrained(
            ProphetNetModelResources::PROPHETNET_LARGE_CNN_DM,
        ));

        let summarization_config = SummarizationConfig {
            model_type: ModelType::ProphetNet,
            model_resource: weights_resource,
            config_resource,
            vocab_resource: vocab_resource.clone(),
            merges_resource: vocab_resource,
            length_penalty: 1.2,
            num_beams: 4,
            no_repeat_ngram_size: 3,
            device: Device::cuda_if_available(),
            ..Default::default()
        };
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
