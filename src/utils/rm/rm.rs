// https://man.archlinux.org/man/rm.1.en

use std::{ fs, error::Error };

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true, num_args(1..))]
    sources: Vec<String>,

    #[clap(short, long)]
    recursive: bool
}

fn rm(args: Args) -> Result<(), Box<dyn Error>> {
    for src in &args.sources {
        if args.recursive {
            let meta = fs::metadata(src)?;

            if meta.is_dir() {
                fs::remove_dir_all(src)?;
            } else {
                fs::remove_file(src)?;
            }
        } else {
            fs::remove_file(src)?;
        }

        println!("Removed '{}'", src);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    rm(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_file() {
        let _ = fs::File::create("a");
        let args = Args {
            sources: vec!["a".to_string()],
            recursive: false
        };

        assert!(rm(args).is_ok());
    }

    #[test]
    fn test_remove_dir() {
        let _ = fs::create_dir("b");
        let args = Args {
            sources: vec!["b".to_string()],
            recursive: true
        };
        
        assert!(rm(args).is_ok());
    }
}