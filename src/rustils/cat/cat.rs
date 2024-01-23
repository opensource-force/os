// https://man.archlinux.org/man/cat.1

use std::env;
use std::io;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut buffer = String::new();
  let stdin = io::stdin();

  if args.len() == 1 {
    loop {
      let _ = stdin.read_line(&mut buffer);
      println!("{}", buffer);
    }
  } else {
    for arg in &args[1..] {
      match fs::read_to_string(arg) {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error reading {}: {}", arg, e)
      }
    }
  }
}