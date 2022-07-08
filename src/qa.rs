#![forbid(unsafe_code)]
use crate::bert::{Bert, Message as M};
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
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct QAer {}

impl Bert for QAer {
    fn runner(receiver: mpsc::Receiver<M>) -> Result<()> {
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
}
