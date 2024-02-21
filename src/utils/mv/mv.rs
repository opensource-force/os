// https://man.archlinux.org/man/mv.1.en

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn mv(src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
    fs::metadata(src)?;

    let src_path = Path::new(src);
    let mut dst_path = PathBuf::from(dst);

    if dst_path.is_dir() {
        dst_path.push(src)
    }
    
    fs::rename(src_path, &dst_path)?;
    println!("Moved '{}' to '{}'", src, dst_path.to_string_lossy());

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = clop::get_opts();

    if opts.scrap.len() < 2 {
        panic!("Usage: mv [OPTION]... <TARGET>... <DESTINATION>");
    }

    let dst = opts.scrap.last().unwrap();

    for arg in &opts.scrap[..opts.scrap.len() - 1] {
        mv(arg, dst)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_file() {
        let _ = fs::File::create("a");

        assert!(mv("a", "b").is_ok());

        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_move_directory() {
        let _ = fs::create_dir("c");

        assert!(mv("c", "d").is_ok());

        let _ = fs::remove_dir("d");
    }

    #[test]
    fn test_move_to_destination() {
        let _ = fs::File::create("e");
        let _ = fs::create_dir("f");

        assert!(mv("e", "f").is_ok());

        let _ = fs::remove_dir_all("f");
    }
}