// https://man.archlinux.org/man/ls.1.en

use std::error::Error;
use std::fs;

fn ls(dst: &str, is_hidden: bool) -> Result<(), Box<dyn Error>> {
    let dirs = fs::read_dir(dst)?;
    
    for dir in dirs {
        let file_path = dir.unwrap().path();
        let file_name = file_path.strip_prefix(dst)?.file_name().unwrap().to_string_lossy();

        if is_hidden || !file_name.starts_with(".") {
            println!("{}", file_name);
        }
    }

    Ok(())
}

fn main() {
    let mut opts = clop::get_opts();
    let is_hidden = opts.has(&["a", "all"], false).is_ok();

    if opts.scrap.len() == 0 {
        opts.scrap.push("./".to_string());
    }

    for arg in &opts.scrap {
        let _ = ls(arg, is_hidden);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dir() {
        assert!(ls("./", false).is_ok());
    }

    #[test]
    fn test_list_dir_hidden() {
        assert!(ls("./", true).is_ok());
    }
}