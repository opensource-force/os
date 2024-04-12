// https://man.archlinux.org/man/mkdir.1

use std::{ fs, error::Error };

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true, num_args(1..))]
    dirs: Vec<String>,

    #[clap(short, long)]
    parent: bool
}

fn mkdir(args: Args) -> Result<(), Box<dyn Error>> {
    for dir in &args.dirs {
        if args.parent {
            fs::create_dir_all(dir)?;
        } else {
            fs::create_dir(dir)?;
        }
        println!("Created directory '{}'", dir);
    }
    
    Ok(())
}

fn main() {
    let args = Args::parse();

    mkdir(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_dir() {
        let args = Args {
            dirs: vec!["a".to_string()],
            parent: false
        };

        assert!(mkdir(args).is_ok());

        let _ = fs::remove_dir("a");
    }

    #[test]
    fn test_make_parent_dir() {
        let args = Args {
            dirs: vec!["b/c".to_string()],
            parent: true
        };

        assert!(mkdir(args).is_ok());

        let _ = fs::remove_dir_all("b");
    }
}