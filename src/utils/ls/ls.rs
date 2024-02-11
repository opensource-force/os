// https://man.archlinux.org/man/ls.1.en

use std::fs;

fn list_dir(dst: &str, is_hidden: bool) -> bool {
    match fs::read_dir(dst) {
        Ok(paths) => {
            for path in paths {
                if let Ok(file) = path.unwrap().path().strip_prefix(dst) {
                    let file_name = file.file_name().unwrap().to_string_lossy();
                  
                    if is_hidden || !file_name.starts_with(".") {
                        println!("{}", file.display());
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Cannot access '{}': {}", dst, e);
            return false
        }
    }

    return true
}

fn main() {
    let mut opts = clop::get_opts();
    let is_hidden = opts.has(&["a", "all"], None);

    if opts.scrap.len() == 0 {
        opts.scrap.push("./".to_string());
    }

    for arg in &opts.scrap {
        list_dir(arg, is_hidden);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dir() {
        assert!(list_dir("./", false));
    }

    #[test]
    fn test_list_dir_hidden() {
        assert!(list_dir("./", true));
    }
}