// https://man.archlinux.org/man/cat.1

use std::io;
use std::fs;

fn main() {
    let opts = clop::get_opts();
    let mut buffer = String::new();
    let stdin = io::stdin();

    if opts.scrap.len() == 0 {
        loop {
            let _ = stdin.read_line(&mut buffer);
            println!("{}", buffer);
        }
    } else {
        for arg in &opts.scrap {
            match fs::read_to_string(arg) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("Error reading {}: {}", arg, e)
            }
        }
    }
}