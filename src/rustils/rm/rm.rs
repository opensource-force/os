// https://man.archlinux.org/man/rm.1.en
// -r | remove_directory

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  for arg in &args[1..] {
    match fs::remove_file(arg) {
      Ok(_) => println!("Removed {}", arg),
      Err(e) => println!("Error removing {}: {}", arg, e)
    }
  }
}