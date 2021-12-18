// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const STANDARD_PRODUCTION: u32 = 221;
const SUCCESS_90: f64 = 0.9;
const SUCCESS_77: f64 = 0.77;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    loop {

        match speed {
            1..=4 => break speed as f64 * STANDARD_PRODUCTION as f64,
            5..=8 => break speed as f64 * STANDARD_PRODUCTION as f64 * SUCCESS_90,
            9..=10 => break speed as f64 * STANDARD_PRODUCTION as f64 * SUCCESS_77,
            _ => {            
                println!("Error: Speed must be within 1 and 10. Please enter another number");
                continue; 
            }
        }
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
        loop {

        match speed {
            1..=10 => break production_rate_per_hour(speed) as u32 / 60,
            _ => {            
                println!("Error: Speed must be within 1 and 10. Please enter another number");
                continue; 
            }
        }
    }
}
