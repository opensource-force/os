// https://man.archlinux.org/man/ln.1

use std::error::Error;
use std::fs;
use std::os::unix::fs as unix_fs;

fn ln(src: &str, dst: &str, is_symbolic: bool) -> Result<(), Box<dyn Error>> {
    fs::metadata(src)?;

    if is_symbolic {
        unix_fs::symlink(src, dst)?;
    } else {
        fs::hard_link(src, dst)?;
    }
    println!("Created link '{}' from '{}'", dst, src);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = clop::get_opts();
    let is_symbolic = opts.has(&["s", "symbolic"], false).is_ok();

    if opts.scrap.len() != 2 {
        panic!("Usage: ln [OPTION]... <TARGET> [LINK_NAME]");
    }

    ln(&opts.scrap[0], &opts.scrap[1], is_symbolic)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hardlink() {
        let _ = fs::File::create("a");
        
        assert!(ln("a", "b", false).is_ok());

        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_create_symlink() {
        let _ = fs::File::create("c");

        assert!(ln("c", "d", true).is_ok());

        let _ = fs::remove_file("c");
        let _ = fs::remove_file("d");
    }
}