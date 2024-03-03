// https://man.archlinux.org/man/cp.1

use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::path::Path;

fn copy_file(src_path: Cow<Path>, dst_path: Cow<Path>) -> Result<(), Box<dyn Error>> {
    fs::copy(&src_path, &dst_path)?;
    println!("Copied '{}' to '{}'", src_path.display(), dst_path.display());
    Ok(())
}

fn copy_directory(src_path: Cow<Path>, dst_path: Cow<Path>, is_recursive: bool) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(&dst_path)?;

    for entry in fs::read_dir(&src_path)? {
        let entry_path = Cow::from(
            entry?.path()
        );

        let new_dst = match entry_path.file_name() {
            Some(file_name) => Cow::from(dst_path.join(file_name)),
            None => dst_path.clone(),  // this should never happen
        };

        if entry_path.is_file() {
            copy_file(entry_path, new_dst)?;
        } else if entry_path.is_dir() && is_recursive {
            copy_directory(entry_path, new_dst, is_recursive)?;
        } else {
            // invalid path (shouldn't happen as it's a result of fs::read_dir, but I am not sure)
            // My suggestion is to ignore this case and print a warning.
        }
    }

    Ok(())
}

fn cp(src: &str, dst: &str, is_recursive: bool) -> Result<(), Box<dyn Error>> {
    let src_path = Cow::from(Path::new(src));
    let mut dst_path = Cow::from(Path::new(dst));

    // ToDo: Handle cases when passing invalid paths and add some errors on '// invalid path'.
    // For example:
    // - src = "/directory"
    // - dst = "file.txt"
    //
    // I am not sure what behavior is desired in such cases,
    // so I am leaving it for @wick3dr0se to decide and implement

    if dst_path.is_dir() {
        dst_path.to_mut().push(&src_path);
    }

    if src_path.is_dir() {
        dst_path.to_mut().push(&src_path);
        copy_directory(src_path, dst_path, is_recursive)
    } else if src_path.is_file() {
        copy_file(src_path, dst_path)
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