// https://man.archlinux.org/man/chown.1

use std::{
    fs,
    os::unix::fs as unix_fs,
    error::Error
};

use lib::files::{user_to_uid, group_to_gid};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    owner: String,
    #[clap(required = true, num_args(1..))]
    sources: Vec<String>
}

// chown <USER>[:GROUP] <SOURCES>...
fn chown(args: Args) -> Result<(), Box<dyn Error>> {
    for src in &args.sources {
        fs::metadata(src)?;

        let (user, some_grp): (&str, Option<&str>) = if let Some(sep) = args.owner.find(':') {
            let (u, g) = &args.owner.split_at(sep);
            (u, Some(&g[1..]))
        } else {
            (&args.owner, None)
        };

        let uid = user_to_uid(&user)?;
        let some_gid = some_grp.as_ref().and_then(|g| group_to_gid(g).ok());
    
        unix_fs::chown(&src, Some(uid), some_gid)?;
        
        println!("Ownership '{}' set on '{}'", &user, src);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    chown(args).unwrap();
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_change_file_ownership() {
        let _ = fs::File::create("a");
        let args = Args {
            owner: env::var("USER").unwrap(),
            sources: vec!["a".to_string()]
        };

        assert!(chown(args).is_ok());

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_change_dir_ownership() {
        let _ = fs::create_dir("b");
        let user = env::var("USER").unwrap();
        let args = Args {
            owner: user.clone(),
            sources: vec!["b".to_string()]
        };

        assert!(chown(args).is_ok());

        let _ = fs::remove_dir("b");
    }

    #[test]
    fn test_change_file_user_ownership() {
        let _ = fs::File::create("c");
        let args = Args {
            owner: env::var("USER").unwrap(),
            sources: vec!["c".to_string()]
        };

        assert!(chown(args).is_ok());

        let _ = fs::remove_file("c");
    }
}