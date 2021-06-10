use std::time::Duration;

use async_framework::prelude::*;
use tokio::time::sleep;

enum SensorsCheck {
    BatteryLevel(u8),
    TemperatureLevel(u32),
    HumidityLevel(f64),
}

#[allow(dead_code)]
enum MachineLearningAnalysis {
    EMG {
        upper_arm_voltage: f64,
        lower_arm_voltage: f64,
    },
}

#[allow(dead_code)]
enum SystemJob {
    SensorsCheck(SensorsCheck),
    MachineLearningAnalysis(MachineLearningAnalysis),
}

async fn get_battery_levels(contacts: Contacts<MS>) -> SystemJob {
    // doing a bunch of GPIO interfacing
    // assume this takes a somewhat long time
    sleep(Duration::from_millis(5_000u64)).await;

    let battery_levels = 97u8;

    contacts.send("user", MS).unwrap();

    SystemJob::SensorsCheck(SensorsCheck::BatteryLevel(battery_levels))
}

async fn get_temperature_levels(_: Contacts<MS>) -> SystemJob {
    // doing a bunch of GPIO interfacing
    // assume this doesn't take too long
    sleep(Duration::from_millis(2_000u64)).await;

    let temperature_levels = 25u32;

    SystemJob::SensorsCheck(SensorsCheck::TemperatureLevel(temperature_levels))
}

async fn get_humidity_levels(_: Contacts<MS>) -> SystemJob {
    // doing a bunch of GPIO interfacing
    // assume this takes a very, VERY long
    sleep(Duration::from_millis(10_000u64)).await;

    let humidity_levels = 101.101f64;

    SystemJob::SensorsCheck(SensorsCheck::HumidityLevel(humidity_levels))
}

#[derive(Clone)]
struct MS;

async fn handler1(_: Contacts<MS>, _: MS) { println!("got a message") }
async fn handler2(_: Contacts<MS>, _: MS) { println!("got a message") }

fn main() {
    // create all jobs
    let battery_job = Job::from_function(get_battery_levels);
    let temperature_job = Job::from_function(get_temperature_levels);
    let humidity_job = Job::from_function(get_humidity_levels);
    let spacer = Job::from_spacer(3_000u64);

    let critical_routine_builder = {
        let mut rb = RoutineBuilder::with_capacity(2usize);
        rb.push(battery_job);
        rb.push(temperature_job);
        rb.push(humidity_job);
        rb.push(spacer.clone());

        rb
    };

    let user_routine_builder = {
        let rb = RoutineBuilder::with_capacity(2usize);

        rb
    };

    let mut critical_component_builder =
        ComponentBuilder::new("critical", critical_routine_builder, handler1).unwrap();

    let mut user_component_builder =
        ComponentBuilder::new("user", user_routine_builder, handler2).unwrap();

    // adding components to each others' contacts
    critical_component_builder.add_component(&user_component_builder);
    user_component_builder.add_component(&critical_component_builder);

    let mut sb = SystemBuilder::with_capacity(2usize);
    sb.push(critical_component_builder);
    sb.push(user_component_builder);

    sb.build().unwrap().run();
}
