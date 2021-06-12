// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::time::Duration;

use r_prime::{
    component::state_store::StateStore,
    prelude::*,
};
use tokio::time::sleep;

// async jobs
async fn test1(_: Port<MS>) {
    println!("TEST1");
    sleep(Duration::from_millis(3000u64)).await;
    println!("finished computation");
}
async fn test2(_: Port<MS>) {
    println!("TEST2");
}

// Messaging System
#[derive(Clone)]
enum MS {}

// State Store
struct Store;
impl Store {
    fn update(&mut self, _: u8) {}
}
impl StateStore for Store {}

// async handler for all incoming messages
async fn handler1(_: Port<MS>, _: MS) {}

fn main() {
    let mut cb1 = ComponentBuilder::new("c1", Box::new(Store), handler1).unwrap();

    #[allow(unused)]
    let mut c = {
        let rb = cb1.routine_builder();
        rb.push(Job::from_function(test1));
        rb.push(Job::from_function(test2));
        rb.push(Job::from_spacer(1000u64));

        let pb = cb1.port_builder();

        cb1.build()
    }
    .unwrap();

    c.state_store::<Store>().update(0u8);
}
