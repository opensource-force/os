// https://man.archlinux.org/man/cp.1

use std::{
    borrow::Cow,
    path::Path,
    error::Error
};

use lib::files::{copy_file, copy_directory};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true, num_args(1..))]
    sources: Vec<String>,
    #[clap(required = true)]
    dest: String,

    #[clap(short, long)]
    recursive: bool
}

fn cp(args: Args) -> Result<(), Box<dyn Error>> {
    for src in &args.sources {
        let src_path = Path::new(src);
        let mut dst_path = Cow::from(Path::new(&args.dest));

        if dst_path.is_dir() {
            dst_path.to_mut().push(src_path);
        }

        if src_path.is_dir() {
            copy_directory(src_path, dst_path, args.recursive)?
        } else if src_path.is_file() {
            copy_file(src_path, dst_path)?
        }

        println!("Copied '{}' to '{}'", src, &args.dest);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    cp(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_copy_file() {
        let _ = fs::File::create("a");
        let args = Args {
            sources: vec!["a".to_string()],
            dest: "b".to_string(),
            recursive: false
        };
        
        assert!(cp(args).is_ok());
        
        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_copy_directory() {
        let _ = fs::create_dir("c");
        let args = Args {
            sources: vec!["c".to_string()],
            dest: "d".to_string(),
            recursive: true
        };

        assert!(cp(args).is_ok());
        
        let _ = fs::remove_dir("c");
        let _ = fs::remove_dir("d");
    }

    #[test]
    fn test_copy_to_destination() {
        let _ = fs::File::create("e");
        let _ = fs::create_dir("f");
        let args = Args {
            sources: vec!["e".to_string()],
            dest: "f".to_string(),
            recursive: false
        };
        
        assert!(cp(args).is_ok());
        
        let _ = fs::remove_file("e");
        let _ = fs::remove_dir_all("f");
    }
}