use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;
use std::process;

fn create_link(target_path: &str, link_path: &str, is_symbolic: bool) {
    if is_symbolic {
        match unix_fs::symlink(target_path, &link_path) {
            Ok(_) => println!("Created symlink {}", link_path),
            Err(e) => {
                eprintln!("Error creating symlink {}", e);
                process::exit(1);
            }
        }
    } else {
        match fs::hard_link(target_path, &link_path) {
            Ok(_) => println!("Created hardlink {}", link_path),
            Err(e) => {
                eprintln!("Error creating hardlink {}", e);
                process::exit(1);
            }
        }
    }
}

fn main() {
    let mut opts = clop::get_opts();
    let is_symbolic = opts.has(&["s", "symbolic"], None);

    let (target_path, link_path): (&String, String) = match opts.scrap.len() {
        2 => (&opts.scrap[0], opts.scrap[1].to_string()),
        1 => {
            let link_name = Path::new(&opts.scrap[0]).file_name().unwrap_or_default().to_string_lossy().to_string();
            (&opts.scrap[0], link_name)
        },
        _ => {
            eprintln!("Incorrect number of arguments");
            process::exit(1);
        }
    };

    create_link(target_path, &link_path, is_symbolic);
}

#[cfg(test)]
mod tests {
    use fs;
    use create_link;

    #[test]
    fn test_hardlink() {
        let _ = fs::File::create("a");

        create_link("a", "b", false);

        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_symlink() {
        let _ = fs::File::create("a");

        create_link("a", "b", true);

        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }
}
