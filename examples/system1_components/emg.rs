use r_prime::prelude::*;

use crate::system1_components::fa::FaMsgs;
use crate::system1_components::System1Error;
use crate::Txs;

pub async fn main(txs: Txs) -> Result<(), System1Error> {
    loop {
        sleep(500u64).await;
        txs.fa_tx.send(FaMsgs::Ping).await?;
    };
}

fn main() {
    // loop {
    //     let bp = check_battery();
    //     if bp >= 20 {
    //         check_sensor1();
    //         check_sensor2();
    //         check_sensor3();

    //         let finger_positions = run_analytics();
    //         update_motors(finger_positions);
    //     }

    //     else { thread::sleep(5); };
    // };

    loop {
        let jh1 = tokio::spawn(async {
            check_sensor1();
            check_sensor2();
            check_sensor3();
            send_sensor_data();
        });
        let jh2 = tokio::spawn(async {
            receive_analytics_state();
            set_motors_and_servos();
        });

        

        // check_sensor1().await;
        // check_sensor2().await;
        // check_sensor3().await;

        // send_sensor_data().await;

        // receive_analytics_state().await;
        // set_motors_and_servos().await;
    }
}
