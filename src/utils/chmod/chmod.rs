// https://man.archlinux.org/man/chmod.1

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    error::Error,
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    mode: u32,
    #[clap(required = true, num_args(1..))]
    sources: Vec<String>
}

fn chmod(args: Args) -> Result<(), Box<dyn Error>> {
    for src in &args.sources {
        fs::set_permissions(src, fs::Permissions::from_mode(args.mode))?;
    
        println!("Mode '{}' set on '{}'", &args.mode, src);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    
    chmod(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_file_mode() {
        let _ = fs::File::create("a");
        let args = Args {
            mode: 755,
            sources: vec!["a".to_string()]
        };

        assert!(chmod(args).is_ok());

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_change_dir_mode() {
        let _ = fs::create_dir("b");
        let args = Args {
            mode: 644,
            sources: vec!["b".to_string()]
        };

        assert!(chmod(args).is_ok());

        let _ = fs::remove_dir("b");
    }
}