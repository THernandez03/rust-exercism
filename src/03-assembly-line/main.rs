const CARS_RATE: f64 = 221.0;
const MINUTES_PER_HOURS: f64 = 60.0;

fn production_rate_per_hour(speed: u8) -> f64 {
    let production_count = speed as f64 * CARS_RATE;
    let production_rate = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9 | 10 => 0.77,
        _ => panic!("Unexpected speed: {}", speed),
    };
    production_count * production_rate
}

fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / MINUTES_PER_HOURS).trunc() as u32
}

pub fn main() -> String {
    working_items_per_minute(6).to_string()
}
