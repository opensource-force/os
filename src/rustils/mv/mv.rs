// https://man.archlinux.org/man/mv.1.en

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let dest_path = args.last().unwrap();
  
  for arg in &args[1..args.len() - 1] {
    match fs::rename(arg, dest_path) {
      Ok(_) => println!("Moved {} to {}", arg, dest_path),
      Err(e) => eprintln!("Error moving {}: {}", arg, e)
    }
  }
}