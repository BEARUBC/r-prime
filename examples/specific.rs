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

enum SystemJob {
    SensorsCheck(SensorsCheck),
    MachineLearningAnalysis(MachineLearningAnalysis),
}

async fn get_battery_levels(_: Contacts<MS>) -> SystemJob {
    println!("getting battery percentage");

    // doing a bunch of GPIO interfacing
    // assume this takes a somewhat long time
    sleep(Duration::from_millis(5_000u64)).await;

    println!("got battery percentage");
    let battery_levels = 97u8;

    SystemJob::SensorsCheck(SensorsCheck::BatteryLevel(battery_levels))
}

async fn get_temperature_levels(_: Contacts<MS>) -> SystemJob {
    println!("getting temperature levels");

    // doing a bunch of GPIO interfacing
    // assume this doesn't take too long
    sleep(Duration::from_millis(2_000u64)).await;

    println!("got temperature levels");
    let temperature_levels = 25u32;

    SystemJob::SensorsCheck(SensorsCheck::TemperatureLevel(temperature_levels))
}

async fn get_humidity_levels(_: Contacts<MS>) -> SystemJob {
    println!("getting humidity levels");

    // doing a bunch of GPIO interfacing
    // assume this takes a very, VERY long
    sleep(Duration::from_millis(10_000u64)).await;

    println!("got humidity levels");
    let humidity_levels = 101.101f64;

    SystemJob::SensorsCheck(SensorsCheck::HumidityLevel(humidity_levels))
}

async fn run_emg_analytics(_: Contacts<MS>) -> SystemJob {
    println!("running emg analytics");

    // running a lot of Python
    // assume this takes a very, VERY long
    sleep(Duration::from_millis(20_000u64)).await;

    println!("got emg analytics");
    let upper_arm_voltage = 0.10f64;
    let lower_arm_voltage = 0.07f64;

    SystemJob::MachineLearningAnalysis(MachineLearningAnalysis::EMG {
        upper_arm_voltage,
        lower_arm_voltage,
    })
}

#[derive(Clone)]
struct MS;

async fn handler(_: Contacts<MS>, _: MS) {}

fn main() {
    // create all jobs
    let battery_job = Job::from_function(get_battery_levels);
    let temperature_job = Job::from_function(get_temperature_levels);
    let humidity_job = Job::from_function(get_humidity_levels);
    let emg_job = Job::from_function(run_emg_analytics);
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
        let mut rb = RoutineBuilder::with_capacity(2usize);
        rb.push(emg_job);
        rb.push(spacer.clone());

        rb
    };

    let mut critical_component_builder =
        ComponentBuilder::new("critical", critical_routine_builder, handler).unwrap();

    let mut user_component_builder =
        ComponentBuilder::new("user", user_routine_builder, handler).unwrap();

    // adding components to each others' contacts
    critical_component_builder.add_component(&user_component_builder);
    user_component_builder.add_component(&critical_component_builder);

    let mut sb = SystemBuilder::with_capacity(2usize);
    sb.push(critical_component_builder);
    sb.push(user_component_builder);

    sb.build().unwrap().run();
}
