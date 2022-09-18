#![allow(unconditional_recursion)]

pub fn main() {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();

  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

  match input.trim(){
    _ => {},
  }

  main();
}
