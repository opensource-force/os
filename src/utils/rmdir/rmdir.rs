//https://man.archlinux.org/man/rmdir.1
// -p | remove parent directories

use std::{ fs, error::Error };

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true, num_args(1..))]
    dirs: Vec<String>
}

fn rmdir(args: Args) -> Result<(), Box<dyn Error>> {
    for dir in &args.dirs {
        fs::remove_dir(dir)?;
        
        println!("Removed directory '{}'", dir);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    
    rmdir(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dir() {
        let _ = fs::create_dir("a");
        let args = Args {
            dirs: vec!["a".to_string()]
        };

        assert!(rmdir(args).is_ok());
    }
}