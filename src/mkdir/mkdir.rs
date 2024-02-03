// https://man.archlinux.org/man/mkdir.1

use std::fs;

fn make_dir(dir: &str, is_parent: Option<bool>) -> bool {
    let result = match is_parent {
        Some(true) => fs::create_dir_all(dir),
        None | Some(false) => fs::create_dir(dir)
    };
    
    match result {
        Ok(_) => println!("Created directory: {}", dir),
        Err(e) => {
            eprintln!("Error creating directory {}: {}", dir, e);
            return false
        }
    }

    return true
}

fn main() {
    let mut opts = clop::get_opts();
    let is_parent = opts.has(&["p", "parents"], None);

    for arg in &opts.scrap {
        make_dir(arg, Some(is_parent));
    }
}

#[cfg(test)]
mod tests {
    use fs;
    use make_dir;

    #[test]
    fn test_make_dir() {
        assert!(make_dir("a", None));

        let _ = fs::remove_dir("a");
    }

    #[test]
    fn test_make_dir_parent() {
        assert!(make_dir("b/c", Some(true)));

        let _ = fs::remove_dir_all("b");
    }
}