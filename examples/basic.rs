use std::time::Duration;

use r_prime::prelude::*;
use tokio::time::sleep;

async fn test1(_: Port<MS>) {
    println!("TEST1");
    sleep(Duration::from_millis(3000u64)).await;
    println!("finished computation");
}
async fn test2(_: Port<MS>) {
    println!("TEST2");
}

#[derive(Clone)]
struct MS;

async fn handler1(_: Port<MS>, _: MS) {}

fn main() {
    let mut cb1 = ComponentBuilder::new("c1", handler1).unwrap();

    #[allow(unused)]
    let c = {
        let rb = cb1.routine_builder();
        rb.push(Job::from_function(test1));
        rb.push(Job::from_function(test2));
        rb.push(Job::from_spacer(1000u64));

        let pb = cb1.port_builder();

        cb1.build()
    };
}
