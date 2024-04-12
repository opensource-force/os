// https://man.archlinux.org/man/touch.1.en

use std::{ fs, error::Error };

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true, num_args(1..))]
    files: Vec<String>
}

fn touch(args: Args) -> Result<(), Box<dyn Error>> {
    for file in &args.files {
        let new_file = fs::File::create(file)?;
        new_file.set_len(0)?;
        
        println!("Created '{}'", file);
    }
    
    Ok(())
}

fn main() {
    let args = Args::parse();

    touch(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_file() {
        let args = Args {
            files: vec!["a".to_string()]
        };

        assert!(touch(args).is_ok());

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_update_timestamp() {
        let _ = fs::File::create("b");
        let args = Args {
            files: vec!["b".to_string()]
        };

        assert!(touch(args).is_ok());

        let _ = fs::remove_file("b");
    }
}