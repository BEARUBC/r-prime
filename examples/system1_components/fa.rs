use tokio::sync::mpsc::Receiver;

use crate::system1_components::System1Error;
use crate::Txs;

fn do_something() {
    // does something
}

pub async fn main(_: Txs, mut rx: Receiver<FaMsgs>) -> Result<(), System1Error> {
    while let Some(msg) = rx.recv().await {
        match msg {
            FaMsgs::Ping => println!("got ping"),
        };
    };

    // loop {
    //     sleep(10_000).await;
    //     do_something();
    //     if (rx.has_message()) {
    //         let Some(msg) = rx.recv().await;
    //         match msg {
    //             FaMsgs::Ping => println!("got ping"),
    //         };
    //     };
    // };

    Ok(())
}

#[derive(Debug)]
pub enum FaMsgs {
    Ping,
}
