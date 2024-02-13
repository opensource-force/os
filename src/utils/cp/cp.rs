use std::fs;
use std::path::Path;

fn copy_file(src: &str, dst: &str, is_recursive: bool) -> bool {
    if let Err(e) = fs::metadata(src) {
        eprintln!("Cannot stat '{}': {}", src, e);
        return false
    }
    
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);
    let dest = if dst_path.is_dir() {
        dst_path.join(src)
    } else {
        dst_path.to_path_buf()
    };
    let res = if is_recursive {
        let _ = fs::create_dir(&dest);

        if let Ok(entries) = fs::read_dir(&src_path) {
            for entry in entries {
                if let Ok(file) = entry {
                    let new_dst = dest.join(file.file_name());
                    copy_file(&file.path().to_string_lossy(), &new_dst.to_string_lossy(), true);
                }
            }
        }
        Ok(())
    } else {
        fs::copy(&src_path, &dest).map(|_| ())
    };

    if let Err(e) = res {
        eprintln!("Error copying '{}': {}", src, e);
        return false
    }

    println!("Copied '{}' to '{}'", src, dest.to_string_lossy());
    return true
}

fn main() {
    let mut opts = clop::get_opts();

    if opts.scrap.len() < 2 {
        panic!("Usage: cp [OPTION]... <TARGET>... <DESTINATION>");
    }

    let is_recursive = opts.has(&["r", "recursive"], false).is_ok();

    if let Some(dst) = opts.scrap.last() {
        for arg in &opts.scrap[..opts.scrap.len() - 1] {
            copy_file(arg, dst, is_recursive);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file() {
        let _ = fs::File::create("a");
        
        assert!(copy_file("a", "b", false));
        
        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_copy_directory() {
        let _ = fs::create_dir("c");

        assert!(copy_file("c", "d", true));
        
        let _ = fs::remove_dir("c");
        let _ = fs::remove_dir("d");
    }

    #[test]
    fn test_copy_to_destination() {
        let _ = fs::File::create("e");
        let _ = fs::create_dir("f");
        
        assert!(copy_file("e", "f", false));
        
        let _ = fs::remove_file("e");
        let _ = fs::remove_dir_all("f");
    }
}