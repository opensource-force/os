// https://man.archlinux.org/man/cp.1
// -r | copy directories

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let dest_path = args.last().unwrap();
  
  for arg in &args[1..args.len() - 1] {
    match fs::copy(arg, dest_path) {
      Ok(_) => println!("Copied {} to {}", arg, dest_path),
      Err(e) => println!("Error copying {}: {}", arg, e)
    }
  }
}