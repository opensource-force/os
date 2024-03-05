// https://man.archlinux.org/man/cp.1

mod io;

use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::path::Path;

fn cp(src: &str, dst: &str, is_recursive: bool) -> Result<(), Box<dyn Error>> {
    let src_path = Path::new(src);
    let mut dst_path = Cow::from(Path::new(dst));

    // ToDo: Handle cases when passing invalid paths and add some errors on '// invalid path'.
    // For example:
    // - src = "/directory"
    // - dst = "file.txt"
    //
    // I am not sure what behavior is desired in such cases,
    // so I am leaving it for @wick3dr0se to decide and implement

    if dst_path.is_dir() {
        dst_path.to_mut().push(src_path);
    }

    if src_path.is_dir() {
        io::copy_directory(src_path, dst_path, is_recursive)
    } else if src_path.is_file() {
        io::copy_file(src_path, dst_path)
    } else {
        // invalid path - src_path doesn't exist
        Ok(())
    }
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