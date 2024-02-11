use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;
use std::process;

fn create_link(src: &str, dst: Option<&str>, is_symbolic: bool) -> bool {
    if let Err(e) = fs::metadata(src) {
        eprintln!("Cannot access '{}': {}", src, e);
        return false
    }

    let link = dst.or(Path::new(src).file_name().and_then(|f| f.to_str())).unwrap();
    let (link_type, res) = if is_symbolic {
        ("symlink", unix_fs::symlink(src, link))
    } else {
        ("hardlink", fs::hard_link(src, link))
    };

    if let Err(e) = res {
        eprintln!("Error creating {} '{}': {}", link_type, link, e);
        return false
    }

    println!("Created {} '{}'", link_type, link);
    return true
}

fn main() {
    let mut opts = clop::get_opts();
    let is_symbolic = opts.has(&["s", "symbolic"], None);

    match opts.scrap.len() {
        2 => create_link(&opts.scrap[0], Some(&opts.scrap[1]), is_symbolic),
        1 => create_link(&opts.scrap[0], None, is_symbolic),
        _ => {
            eprintln!("Usage: ln [OPTION]... <TARGET> <LINK_NAME>");
            process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hardlink() {
        let _ = fs::File::create("a");
        
        assert!(create_link("a", Some("b"), false));

        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_create_symlink() {
        let _ = fs::File::create("c");

        assert!(create_link("c", Some("d"), true));

        let _ = fs::remove_file("c");
        let _ = fs::remove_file("d");
    }
}