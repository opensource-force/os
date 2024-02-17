// https://man.archlinux.org/man/rm.1.en

use std::error::Error;
use std::fs;

fn rm(src: &str, is_recursive: bool) -> Result<(), Box<dyn Error>> {
    if is_recursive {
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

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = clop::get_opts();

    if opts.scrap.len() < 1 {
        panic!("Usage: rm [OPTION]... <TARGET>...");
    }

    let is_recursive = opts.has(&["r", "recursive"], false).is_ok();

    for arg in &opts.scrap {
        rm(arg, is_recursive)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_file() {
        let _ = fs::File::create("a");

        assert!(rm("a", false).is_ok());
    }

    #[test]
    fn test_remove_dir() {
        let _ = fs::create_dir("b");
        
        assert!(rm("b", true).is_ok());
    }
}