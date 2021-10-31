use r_prime::prelude::*;

use crate::system1_components::fa::FaMsgs;
use crate::system1_components::System1Error;
use crate::Txs;

pub async fn main(txs: Txs) -> Result<(), System1Error> {
    loop {
        sleep(500u64).await;
        txs.fa_tx.send(FaMsgs::Ping).await?;
    }
}
