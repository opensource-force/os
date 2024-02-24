// https://man.archlinux.org/man/chown.1

use std::fs;
use std::error::Error;
use std::os::unix::fs as unix_fs;

use lib::files::{user_to_uid, group_to_gid};

fn chown(src: &str, user: &str, group: Option<&str>) -> Result<(), Box<dyn Error>> {
    fs::metadata(src)?;

    let uid = user_to_uid(user)?;
    let some_gid = group.and_then(|g| group_to_gid(g).ok());

    unix_fs::chown(src, Some(uid), some_gid)?;
    println!("Ownership '{}' set on '{}'", user, src);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = clop::get_opts();

    if opts.scrap.len() < 2 {
        panic!("Usage: chown [OPTION]... <USER>[:GROUP] <TARGET>...");
    }

    let (user, some_group): (&str, Option<&str>) = if let Some(sep) = opts.scrap[0].find(':') {
        let (u, g) = opts.scrap[0].split_at(sep);
        (u, Some(&g[1..]))
    } else {
        (&opts.scrap[0], None)
    };

    for arg in &opts.scrap[1..] {
        chown(arg, user, some_group)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_change_file_ownership() {
        let _ = fs::File::create("a");
        let user = env::var("USER").unwrap();

        assert!(chown("a", &user, Some(&user)).is_ok());

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_change_dir_ownership() {
        let _ = fs::create_dir("b");
        let user = env::var("USER").unwrap();

        assert!(chown("b", &user, Some(&user)).is_ok());

        let _ = fs::remove_dir("b");
    }

    #[test]
    fn test_change_file_user_ownership() {
        let _ = fs::File::create("c");
        let user = env::var("USER").unwrap();
        
        assert!(chown("c", &user, None).is_ok());

        let _ = fs::remove_file("c");
    }
}