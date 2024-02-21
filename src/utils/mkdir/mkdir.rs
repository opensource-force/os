// https://man.archlinux.org/man/mkdir.1

use std::error::Error;
use std::fs;

fn mkdir(dst: &str, is_parent: bool) -> Result<(), Box<dyn Error>> {
    if is_parent {
        fs::create_dir_all(dst)?;
    } else {
        fs::create_dir(dst)?;
    }
    println!("Created directory '{}'", dst);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = clop::get_opts();
    
    if opts.scrap.len() < 1 {
        panic!("Usage: mkdir [OPTION]... <DIRECTORY>...");
    }

    let has_parent = opts.has(&["p", "parents"], false).is_ok();

    for arg in &opts.scrap {
        mkdir(arg, has_parent)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_dir() {
        assert!(mkdir("a", false).is_ok());

        let _ = fs::remove_dir("a");
    }

    #[test]
    fn test_make_parent_dir() {
        assert!(mkdir("b/c", true).is_ok());

        let _ = fs::remove_dir_all("b");
    }
}