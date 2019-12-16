use std::time::Duration;
use std::thread;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculated slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Today do {} pushups", simulated_expensive_calculation(intensity));
        println!("Next do {} situps", simulated_expensive_calculation(intensity));
    }
}

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}