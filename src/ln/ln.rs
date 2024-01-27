use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;
use std::process;

fn create_link(target_path: &str, link_path: Option<&str>, is_symbolic: Option<bool>) {
    let link = link_path.or(Path::new(target_path).file_name().and_then(|f| f.to_str())).unwrap();
    let (link_type, result) = match is_symbolic {
        Some(true) => ("symlink", unix_fs::symlink(target_path, link)),
        None | Some(false) => ("hardlink", fs::hard_link(target_path, link))
    };

    match result {
        Ok(_) => println!("Created {}: {}", link_type, link),
        Err(e) => {
            eprintln!("Error creating {}: {}", link_type, e);
            process::exit(1);
        }
    }
}

fn main() {
    let mut opts = clop::get_opts();
    let is_symbolic = opts.has(&["s", "symbolic"], None);

    match opts.scrap.len() {
        2 => create_link(&opts.scrap[0], Some(&opts.scrap[1]), Some(is_symbolic)),
        1 => create_link(&opts.scrap[0], None, Some(is_symbolic)),
        _ => {
            eprintln!("Incorrect number of arguments");
            process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use fs;
    use create_link;

    #[test]
    fn test_hardlink() {
        let _ = fs::File::create("a");
        create_link("a", Some("b"), None);

        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_symlink() {
        let _ = fs::File::create("a");
        create_link("a", Some("b"), Some(true));

        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }
}