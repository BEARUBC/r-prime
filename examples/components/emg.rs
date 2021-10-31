use r_prime::prelude::*;

use crate::Txs;

use super::fa::FaMsgs;

pub async fn main(txs: Txs) -> Result<(), u8> {
    loop {
        sleep(1000u64).await;
        txs.fa_tx.send(FaMsgs::Ping).await.map_err(|_| 3u8)?
    }
}
