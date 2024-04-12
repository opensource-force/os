// https://man.archlinux.org/man/ln.1

use std::{
    fs,
    os::unix::fs as unix_fs,
    error::Error
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    source: String,
    #[clap(required = true)]
    dest: String,

    #[clap(short, long)]
    symbolic: bool
}

fn ln(args: Args) -> Result<(), Box<dyn Error>> {
    fs::metadata(&args.source)?;

    if args.symbolic {
        unix_fs::symlink(&args.source, &args.dest)?;
    } else {
        fs::hard_link(&args.source, &args.dest)?;
    }
    println!("Created link '{}' from '{}'", &args.dest, &args.source);
    
    Ok(())
}

fn main() {
    let args = Args::parse();

    ln(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hardlink() {
        let _ = fs::File::create("a");
        let args = Args {
            source: "a".to_string(),
            dest: "b".to_string(),
            symbolic: false
        };

        assert!(ln(args).is_ok());

        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_create_symlink() {
        let _ = fs::File::create("c");
        let args = Args {
            source: "c".to_string(),
            dest: "d".to_string(),
            symbolic: true
        };

        assert!(ln(args).is_ok());

        let _ = fs::remove_file("c");
        let _ = fs::remove_file("d");
    }
}