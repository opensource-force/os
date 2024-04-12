// https://man.archlinux.org/man/ls.1.en

use std::{ fs, error::Error };

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(default_value = "./", num_args(1..))]
    dests: Vec<String>,

    #[clap(short, long)]
    all: bool
}

fn ls(args: Args) -> Result<(), Box<dyn Error>> {
    for dst in &args.dests {
        let dirs = fs::read_dir(dst)?;
    
        for dir in dirs {
            let file_path = dir.unwrap().path();
            let file_name = file_path.strip_prefix(dst)?.file_name().unwrap().to_string_lossy();
    
            if !file_name.starts_with(".") {
                println!("{}", file_name);
            } else if args.all {
                println!("{}", file_name);
            }
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    ls(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dir() {
        let args = Args {
            dests: vec!["./".to_string()],
            all: false
        };

        assert!(ls(args).is_ok());
    }

    #[test]
    fn test_list_dir_all() {
        let args = Args {
            dests: vec!["./".to_string()],
            all: true
        };

        assert!(ls(args).is_ok());
    }
}