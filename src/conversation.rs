#![forbid(unsafe_code)]
use crate::bert::{Bert, Message as M};
use anyhow::Result;
use rust_bert::pipelines::conversation::{
    ConversationConfig, ConversationManager, ConversationModel,
};
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Communicator {}

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
            let tmp = response.values()
                .map(|x| (*x).to_owned())
                .collect();
            sender.send(tmp).expect("sending results");
        }
        Ok(())
    }
}
