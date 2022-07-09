#![forbid(unsafe_code)]
use crate::{Bert, Message as M};
use anyhow::Result;
use async_trait::async_trait;
use rust_bert::pipelines::conversation::{
    ConversationConfig, ConversationManager, ConversationModel,
};
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Communicator {}

#[async_trait]
impl Bert for Communicator {
    fn runner(receiver: mpsc::Receiver<M>) -> Result<()> {
        let config = ConversationConfig {
            do_sample: false,
            num_beams: 3,
            ..Default::default()
        };
        let model = ConversationModel::new(config)?;
        let mut conversation_manager = ConversationManager::new();
        while let Ok((texts, sender)) = receiver.recv() {
            conversation_manager.create(&texts[0]);
            let response = model.generate_responses(&mut conversation_manager);
            let tmp = response.values().map(|x| (*x).to_owned()).collect();
            sender.send(tmp).expect("sending results");
        }
        Ok(())
    }

    async fn handler(contents: Vec<Vec<String>>) -> Result<Vec<String>> {
        let (_handle, sender) = Self::spawn();
        let response = Self::predict(sender, contents[0].clone()).await?;
        Ok(response)
    }
}
