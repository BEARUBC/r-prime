use std::time::Duration;

use async_framework::prelude::*;
use tokio::time::sleep;

async fn test1(_: Contacts<MS>) -> u32 {
    println!("TEST1");
    sleep(Duration::from_millis(3000u64)).await;
    println!("finished computation");

    0u32
}
async fn test2(_: Contacts<MS>) -> u32 {
    println!("TEST2");

    0u32
}

#[derive(Clone)]
struct MS;

async fn handler(_: Contacts<MS>, _: MS) {}

fn main() {
    let j1 = Job::from_spacer(2000u64);
    let j2 = Job::from_function(test1);

    let j3 = Job::from_spacer(8000u64);
    let j4 = Job::from_function(test2);

    let mut rb1 = RoutineBuilder::<MS, u32>::with_capacity(2usize);
    rb1.push(j1);
    rb1.push(j2);

    let mut rb2 = RoutineBuilder::<MS, u32>::with_capacity(2usize);
    rb2.push(j3);
    rb2.push(j4);

    let mut cb1 = ComponentBuilder::new("component1", rb1, handler).unwrap();

    let mut cb2 = ComponentBuilder::new("component2", rb2, handler).unwrap();

    cb1.add_component(&cb2);
    cb2.add_component(&cb1);

    let mut sb = SystemBuilder::with_capacity(2usize);
    sb.push(cb1);
    sb.push(cb2);

    sb.build().unwrap().run();
}
