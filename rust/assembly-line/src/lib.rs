// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production_rate_per_hour = 221.0;
    let mut success_rate = 1.0;
    if speed > 4 && speed <= 8 {
        success_rate = 0.9;
    }
    if speed > 8 {
        success_rate = 0.77;
    }

    production_rate_per_hour * success_rate * speed as f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production_rate_per_minute = 221.0 / 60.0;
    let mut success_rate = 1.0;
    if speed > 4 && speed <= 8 {
        success_rate = 0.9;
    }
    if speed > 8 {
        success_rate = 0.77;
    }

    let working_items = production_rate_per_minute * success_rate * (speed as f64);

    working_items.floor() as u32
}
