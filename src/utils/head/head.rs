// https://man.archlinux.org/man/head.1.en

use std::{ fs, io, io::BufRead };

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(num_args(1..))]
    files: Vec<String>,

    #[clap(short, long, default_value = "10")]
    lines: u8
}

fn head(args: Args) -> io::Result<()> {
    if args.files.is_empty() {
        let stdin = io::stdin();

        loop {
            let mut buf = String::new();
            
            stdin.read_line(&mut buf)?;
            println!("{}", buf);
        }
    }
    
    for file in &args.files {
        let file = fs::File::open(file);
        let reader = io::BufReader::new(file?);
        let line_count = args.lines;
        let mut lines_read = 0;

        for line in reader.lines() {
            if line_count != 0 && lines_read >= line_count {
                break;
            }
    
            let content = line?;
            println!("{}", content);
            lines_read += 1;
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    head(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_file_start() {
        let _ = fs::File::create("a");
        let _ = fs::write("a", "Hello\nWorld!");
        let args = Args {
            files: vec!["a".to_string()],
            lines: 1
        };

        assert!(head(args).is_ok());

        let _ = fs::remove_file("a");
    }
}