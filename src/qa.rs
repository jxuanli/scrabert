#![forbid(unsafe_code)]
use anyhow::Result;
use rust_bert::longformer::{
    LongformerConfigResources, LongformerMergesResources, LongformerModelResources,
    LongformerVocabResources,
};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::question_answering::{
    QaInput, QuestionAnsweringConfig, QuestionAnsweringModel,
};
use rust_bert::resources::RemoteResource;
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tokio::{sync::oneshot, task};

type Message = (Vec<String>, oneshot::Sender<Vec<String>>);

#[derive(Debug, Clone)]
pub struct QuestionAnswerer {
    sender: mpsc::SyncSender<Message>,
}

impl QuestionAnswerer {
    pub fn spawn() -> (JoinHandle<Result<()>>, QuestionAnswerer) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));
        (handle, QuestionAnswerer { sender })
    }

    fn runner(receiver: mpsc::Receiver<Message>) -> Result<()> {
        let config = QuestionAnsweringConfig::new(
            ModelType::Longformer,
            RemoteResource::from_pretrained(LongformerModelResources::LONGFORMER_BASE_SQUAD1),
            RemoteResource::from_pretrained(LongformerConfigResources::LONGFORMER_BASE_SQUAD1),
            RemoteResource::from_pretrained(LongformerVocabResources::LONGFORMER_BASE_SQUAD1),
            Some(RemoteResource::from_pretrained(
                LongformerMergesResources::LONGFORMER_BASE_SQUAD1,
            )),
            false,
            None,
            false,
        );
    
        let model = QuestionAnsweringModel::new(config)?;
        while let Ok((texts, sender)) = receiver.recv() {
            let input = QaInput {
                question: texts[0].clone(),
                context: texts[1].clone(),
            };
            let qa_ins = model.predict(&[input], 1, 32); 
            let mut tmp = Vec::new();
            tmp.push(qa_ins[0][0].answer.clone());
            sender.send(tmp).expect("sending results");
        }

        Ok(())
    }

    pub async fn predict(&self, texts: Vec<String>) -> Result<Vec<String>> {
        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((texts, sender)))?;
        Ok(receiver.await?)
    }
}
