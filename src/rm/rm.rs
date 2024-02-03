// https://man.archlinux.org/man/rm.1.en

use std::fs;

fn remove(file: &str, is_recursive: bool) -> bool {
    let (file_type, result) = if is_recursive {
        match fs::metadata(file) {
            Ok(meta) => {
                if meta.is_dir() {
                    ("directory", fs::remove_dir_all(file))
                } else {
                    ("file", fs::remove_file(file))
                }
            },
            Err(_) => return false
        }
    } else {
        ("file", fs::remove_file(file))
    };

    if let Err(e) = result {
        eprintln!("Error removing {}: {}", file_type, e);
        return false
    }

    println!("Removed {}: {}", file_type, file);
    return true
}

fn main() {
    let mut opts = clop::get_opts();
    let is_recursive = opts.has(&["r", "recursive"], None);

    for arg in &opts.scrap {
        remove(arg, Some(is_recursive));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_file() {
        let _ = fs::File::create("a");

        assert!(remove("a", Some(false)));
    }

    #[test]
    fn test_remove_dir() {
        let _ = fs::create_dir("b");
        
        assert!(remove("b", Some(true)));
    }
}