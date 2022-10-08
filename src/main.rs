use std::{env, io};

#[path = "./01-hello-world/main.rs"]
mod hello_world;
#[path = "./02-lucian-luscious-lasagna/main.rs"]
mod lucian_luscious_lasagna;
#[path = "./03-assembly-line/main.rs"]
mod assembly_line;
#[path = "./04-health-statistics/main.rs"]
mod health_statistics;
#[path = "./05-semi-structured-logs/main.rs"]
mod semi_structured_logs;
#[path = "./06-sublist/main.rs"]
mod sublist;

fn clear_terminal() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let mut input: String = String::new();

    let exercise = args.get(2);
    let exercise_from_args = exercise.is_some();

    if exercise_from_args {
        input = exercise.unwrap().to_string();
    } else {
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
    }

    clear_terminal();
    match input.as_str() {
        "01" => crate::hello_world::main(),
        "02" => crate::lucian_luscious_lasagna::main(),
        "03" => crate::assembly_line::main(),
        "04" => crate::health_statistics::main(),
        "05" => crate::semi_structured_logs::main(),
        "06" => crate::sublist::main(),
        _ => (),
    }

    if !exercise_from_args {
        main();
    }
}
