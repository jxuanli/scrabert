#![forbid(unsafe_code)]
use crate::{Bert, Message as M};
use anyhow::Result;
use async_trait::async_trait;
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::resources::RemoteResource;
use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Summarizer {}

#[async_trait]
impl Bert for Summarizer {
    fn runner(receiver: mpsc::Receiver<M>) -> Result<()> {
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

    async fn handler(contents: Vec<String>) -> Result<String> {
        let (_handle, sender) = Self::spawn();
        let summarization = Self::predict(sender, contents).await?;
        Ok(summarization)
    }
}
