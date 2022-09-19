#![allow(unconditional_recursion)]

#[path = "./01-hello-world/main.rs"]
mod hello_world;
#[path = "./02-lucian-luscious-lasagna/main.rs"]
mod lucian_luscious_lasagna;
#[path = "./03-assembly-line/main.rs"]
mod assembly_line;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    match input.trim() {
        "01" => println!("{}", crate::hello_world::main()),
        "02" => println!("{}", crate::lucian_luscious_lasagna::main()),
        "03" => println!("{}", crate::assembly_line::main()),
        _ => {}
    }

    main();
}
