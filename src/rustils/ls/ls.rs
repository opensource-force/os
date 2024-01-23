// https://man.archlinux.org/man/ls.1.en
// -a | show hidden files

use std::fs;
use std::env;

fn main() {
  let mut args: Vec<String> = env::args().collect();

  if args.len() == 1 {
    let home = env::var("HOME").unwrap();
    args[0] = home;
  }

  for arg in &args {
    match fs::read_dir(arg) {
      Ok(paths) => {
        for path in paths {
          if let Ok(file) = path.unwrap().path().strip_prefix(arg) {
            let file_name = file.file_name().unwrap().to_string_lossy();
            
            if !file_name.starts_with(".") {
              println!("{}", file.display());
            }
          }
        }
      },
      Err(e) => println!("Error reading {}: {}", arg, e)
    }
  }
}