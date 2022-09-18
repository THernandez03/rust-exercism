fn expected_minutes_in_oven() -> u32 {
  40
}

fn remaining_minutes_in_oven(minutes: u32) -> u32 {
  expected_minutes_in_oven() - minutes
}

fn preparation_time_in_minutes(layers: u32) -> u32 {
  layers * 2
}

fn elapsed_time_in_minutes(layers: u32, minutes: u32) -> u32 {
  expected_minutes_in_oven() - remaining_minutes_in_oven(minutes) + preparation_time_in_minutes(layers)
}

pub fn main() -> String {
  elapsed_time_in_minutes(3, 20).to_string()
}
