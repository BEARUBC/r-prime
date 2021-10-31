use std::error::Error;

use tokio::sync::mpsc::Receiver;

use crate::Txs;

pub async fn main(_: Txs, mut rx: Receiver<FaMsgs>) -> Result<(), Box<dyn Error + Send>> {
    while let Some(msg) = rx.recv().await {
        match msg {
            FaMsgs::Ping => println!("got ping"),
        };
    };

    Ok(())
}

#[derive(Debug)]
pub enum FaMsgs {
    Ping,
}
