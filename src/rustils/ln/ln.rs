// https://man.archlinux.org/man/ln.1
// -s | create sybolic links

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let is_symbolic = args.iter().any(|arg| arg == "-s" || arg == "--symbolic");

  if args.len() == 3 {
    let og = &args[1];
    let link = &args[2];
    
    match fs::hard_link(og, link) {
      Ok(_) => println!("Created hardlink {}", link),
      Err(e) => println!("Error creating hardlink {}", e)
    }
  } else if args.len() == 4 && is_symbolic {
    match fs::symlink(og, link) {
      Ok(_) => println!("Created symlink {}", link),
      Err(e) => println!("Error creating symlink {}", e)
    }
  } else {
    println!("Incorrect number of arguments");
  }
}