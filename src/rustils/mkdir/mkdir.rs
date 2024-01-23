// https://man.archlinux.org/man/mkdir.1
// -p | make parent directories

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  for arg in &args[1..] {
    match fs::create_dir(arg) {
      Ok(_) => println!("Created {}", arg),
      Err(e) => println!("Error creating {}: {}", arg, e)
    }
  }
}