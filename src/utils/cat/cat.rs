// https://man.archlinux.org/man/cat.1

use std::{ fs, io::{self, stdin} };

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(num_args(1..))]
    files: Vec<String>
}

fn cat(args: Args) -> io::Result<()> {
    if args.files.is_empty() {
        loop {
            let mut buf = String::new();
            let _ = stdin().read_line(&mut buf);
    
            println!("{}", buf);
        }
    }

    for file in args.files {
        let content = fs::read_to_string(file)?;
        println!("{}", content);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    cat(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_file() {
        let _ = fs::File::create("a");
        let _ = fs::write("a", "Hello World!");
        let args = Args {
            files: vec!["a".to_string()]
        };

        assert!(cat(args).is_ok());

        let _ = fs::remove_file("a");
    }
}