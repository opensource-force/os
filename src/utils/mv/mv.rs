// https://man.archlinux.org/man/mv.1.en

use std::{
    fs,
    path::{Path, PathBuf},
    error::Error
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true, num_args(1..))]
    sources: Vec<String>,
    #[clap(required = true)]
    dest: String
}

fn mv(args: Args) -> Result<(), Box<dyn Error>> {
    for src in &args.sources {
        fs::metadata(src)?;

        let src_path = Path::new(src);
        let mut dst_path = PathBuf::from(&args.dest);

        if dst_path.is_dir() {
            dst_path.push(src)
        }
        
        fs::rename(&src_path, &dst_path)?;
        println!("Moved '{}' to '{}'", src, dst_path.to_string_lossy());
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    mv(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_file() {
        let _ = fs::File::create("a");
        let args = Args {
            sources: vec!["a".to_string()],
            dest: "b".to_string()
        };

        assert!(mv(args).is_ok());

        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_move_directory() {
        let _ = fs::create_dir("c");
        let args = Args {
            sources: vec!["c".to_string()],
            dest: "d".to_string()
        };

        assert!(mv(args).is_ok());

        let _ = fs::remove_dir("d");
    }

    #[test]
    fn test_move_to_destination() {
        let _ = fs::File::create("e");
        let _ = fs::create_dir("f");
        let args = Args {
            sources: vec!["e".to_string()],
            dest: "f".to_string()
        };

        assert!(mv(args).is_ok());

        let _ = fs::remove_dir_all("f");
    }
}