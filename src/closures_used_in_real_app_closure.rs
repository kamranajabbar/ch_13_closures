use std::time::Duration;
use std::thread;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("Calculated slowly");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    //let expension_result = simulated_expensive_calculation(intensity);
    let expension_closure = |num| {
        println!("Calculated slowly!");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today do {} pushups", expension_closure(intensity));
        println!("Next do {} situps", expension_closure(intensity));
    } 
    else 
    {
        if random_number == 3 
        {
            println!("Take a break today!");
        } 
        else 
        {
            println!("Today, run for {} minutes", expension_closure(intensity));
        }
    }
}

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}