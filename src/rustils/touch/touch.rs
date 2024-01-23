// https://man.archlinux.org/man/touch.1.en

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  
  for arg in &args[1..] {
    match fs::File::create(arg) {
      Ok(file) => {
        println!("Created {}", arg);
        
        let _ = file.set_len(0);
      },
      Err(e) => println!("Error creating {}: {}", arg, e)
    }
  }
}