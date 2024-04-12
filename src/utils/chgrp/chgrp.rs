// https://man.archlinux.org/man/chgrp.1.en

use std::{
    os::unix::fs as unix_fs,
    error::Error
};

use lib::files::group_to_gid;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    group: String,

    #[clap(num_args(1..))]
    sources: Vec<String>
}

fn chgrp(args: Args) -> Result<(), Box<dyn Error>> {
    for src in &args.sources {
        let gid = group_to_gid(&args.group)?;

        unix_fs::chown(src, None, Some(gid))?;
        println!("Group '{}' set on '{}'", &args.group, src);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    chgrp(args).unwrap();
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use super::*;

    #[test]
    fn test_change_file_group_ownership() {
        let _ = fs::File::create("a");
        let args = Args {
            group: env::var("USER").unwrap(),
            sources: vec!["a".to_string()]
        };

        assert!(chgrp(args).is_ok());

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_change_dir_group_ownership() {
        let _ = fs::create_dir("b");
        let args = Args {
            group: env::var("USER").unwrap(),
            sources: vec!["b".to_string()]
        };

        assert!(chgrp(args).is_ok());

        let _ = fs::remove_dir("b");
    }
}