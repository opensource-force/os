// https://man.archlinux.org/man/rm.1.en

use std::fs;

fn remove(file: &str, is_recursive: Option<bool>) -> bool {
    let mut file_type = "file";
    let result = match is_recursive {
        Some(true) => {
            match fs::metadata(file) {
                Ok(meta) => {
                    if meta.is_file() {
                        fs::remove_file(file)
                    } else {
                        file_type = "directory";
                        fs::remove_dir_all(file)
                    }
                },
                Err(_) => return false
            }
        },
        None | Some(false) => fs::remove_file(file)
    };

    match result {
        Ok(_) => println!("Removed {}: {}", file_type, file),
        Err(e) => {
            eprintln!("Error removing {}: {}", file_type, e);
            return false
        }
    }

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
    use fs;
    use remove;

    #[test]
    fn remove_file() {
        let _ = fs::File::create("a");
        assert!(remove("a", Some(false)));
    }

    #[test]
    fn remove_dir() {
        let _ = fs::create_dir("b");
        assert!(remove("b", Some(true)));
    }
}