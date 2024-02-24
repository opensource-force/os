// https://man.archlinux.org/man/chgrp.1.en

use std::error::Error;
use std::os::unix::fs as unix_fs;

use lib::files::group_to_gid;

fn chgrp(src: &str, grp: &str) -> Result<(), Box<dyn Error>> {
    let gid = group_to_gid(grp)?;

    unix_fs::chown(src, None, Some(gid))?;
    println!("Group '{}' set on '{}'", grp, src);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = clop::get_opts();
    
    if opts.scrap.len() < 2 {
        panic!("Usage: chgrp [OPTION]... <GROUP> <TARGET>...");
    }

    for arg in &opts.scrap[1..] {
        chgrp(arg, &opts.scrap[0])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use super::*;

    #[test]
    fn test_change_file_group_ownership() {
        let _ = fs::File::create("a");
        let user = env::var("USER").unwrap();

        assert!(chgrp("a", &user).is_ok());

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_change_dir_group_ownership() {
        let _ = fs::create_dir("b");
        let user = env::var("USER").unwrap();

        assert!(chgrp("b", &user).is_ok());

        let _ = fs::remove_dir("b");
    }
}