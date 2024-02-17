// https://man.archlinux.org/man/cp.1

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn cp(src: &str, dst: &str, is_recursive: bool) -> Result<(), Box<dyn Error>> {
    fs::metadata(src)?;
    
    let src_path = Path::new(src);
    let mut dst_path = PathBuf::from(dst);

    if dst_path.is_dir() {
        dst_path.push(src);
    }
    
    if is_recursive {
        fs::create_dir(&dst_path)?;
        let entries = fs::read_dir(&src_path)?;

        for entry in entries {
            let file = entry?;
            let new_dst = dst_path.join(file.file_name());
            cp(&file.path().to_string_lossy(), &new_dst.to_string_lossy(), true)?;
        }
    } else {
        fs::copy(&src_path, &dst_path)?;
    }
    println!("Copied '{} to '{}'", src, dst);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = clop::get_opts();

    if opts.scrap.len() < 2 {
        panic!("Usage: cp [OPTION]... <TARGET>... <DESTINATION>");
    }

    let is_recursive = opts.has(&["r", "recursive"], false).is_ok();
    let dst = opts.scrap.last().unwrap();
    
    for arg in &opts.scrap[..opts.scrap.len() - 1] {
        cp(arg, dst, is_recursive)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file() {
        let _ = fs::File::create("a");
        
        assert!(cp("a", "b", false).is_ok());
        
        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_copy_directory() {
        let _ = fs::create_dir("c");

        assert!(cp("c", "d", true).is_ok());
        
        let _ = fs::remove_dir("c");
        let _ = fs::remove_dir("d");
    }

    #[test]
    fn test_copy_to_destination() {
        let _ = fs::File::create("e");
        let _ = fs::create_dir("f");
        
        assert!(cp("e", "f", false).is_ok());
        
        let _ = fs::remove_file("e");
        let _ = fs::remove_dir_all("f");
    }
}