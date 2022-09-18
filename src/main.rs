#![allow(unconditional_recursion)]

#[path = "./01-hello-world/main.rs"] mod hello_world;

pub fn main() {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();

  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

  match input.trim(){
    "01" => println!("{}", crate::hello_world::main()),
    _ => {},
  }

  main();
}
