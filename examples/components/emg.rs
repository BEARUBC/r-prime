use std::error::Error;
use r_prime::prelude::*;

use crate::Txs;

use super::fa::FaMsgs;

pub async fn main(txs: Txs) -> Result<(), Box<dyn Error + Send>> {
    loop {
        sleep(1000u64).await;
        txs.fa_tx.send(FaMsgs::Ping).await.map_err::<Box<dyn Error + Send>, _>(|e| Box::new(e))?
    }
}
