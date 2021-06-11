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
async fn test3(_: Port<MS>) {
    println!("TEST3");
}
async fn test4(_: Port<MS>) {
    println!("TEST4");
}

#[derive(Clone)]
struct MS;

#[derive(Clone)]
struct MR;

async fn handler1(_: Port<MS>, _: MS) -> MR { MR }
async fn handler2(_: Port<MS>, _: MS) -> MR { MR }

fn main() -> ! {
    let mut cb1 = ComponentBuilder::new("c1", handler1).unwrap();
    let mut cb2 = ComponentBuilder::new("c2", handler2).unwrap();

    // configuring cb1's routine
    {
        let rb = cb1.routine_builder();
        rb.push(Job::from_function(test1));
        rb.push(Job::from_function(test2));
        rb.push(Job::from_spacer(2000u64));
    };

    // configuring cb2's routine
    {
        let rb = cb2.routine_builder();
        rb.push(Job::from_function(test3));
        rb.push(Job::from_function(test4));
        rb.push(Job::from_spacer(1000u64));
    };

    // adding cb2 to cb1's port
    {
        let pb = cb1.port_builder();
        pb.add_component(&mut cb2);
    };

    // adding cb1 to cb2's port
    {
        let pb = cb2.port_builder();
        pb.add_component(&mut cb1);
    };

    let mut sb = SystemBuilder::with_capacity(2usize);

    sb.push(cb1);
    sb.push(cb2);

    sb.build()
        .unwrap()
        .run()
}
