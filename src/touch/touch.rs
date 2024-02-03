// https://man.archlinux.org/man/touch.1.en

use std::fs;

fn make_file(file: &str) -> bool {
    match fs::File::create(file) {
        Ok(created) => {
            println!("Created: {}", file);
    
            let _ = created.set_len(0);
        },
        Err(e) => {
            eprintln!("Error creating {}: {}", file, e);
            return false
        }
    }

    return true
}

fn main() {
    let opts = clop::get_opts();
    
    for arg in &opts.scrap {
        make_file(arg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_file() {
        assert!(make_file("a"));

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_update_timestamp() {
        let _ = fs::File::create("b");

        assert!(make_file("b"));

        let _ = fs::remove_file("b");
    }
}