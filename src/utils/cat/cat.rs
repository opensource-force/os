// https://man.archlinux.org/man/cat.1

use std::io;
use std::fs;

fn echo_file(file: &str) -> bool {
    match fs::read_to_string(file) {
        Ok(content) => println!("{}", content),
        Err(e) => {
            eprintln!("Cannot access '{}': {}", file, e);
            return false
        }
    }

    return true
}

fn main() {
    let opts = clop::get_opts();
    let stdin = io::stdin();

    if opts.scrap.len() == 0 {
        loop {
            let mut buffer = String::new();
            let _ = stdin.read_line(&mut buffer);
    
            println!("{}", buffer);
        }
    } else {
        for arg in &opts.scrap {
            echo_file(arg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_file() {
        let _ = fs::File::create("a");
        let _ = fs::write("a", "Hello World!");

        assert!(echo_file("a"));

        let _ = fs::remove_file("a");
    }
}