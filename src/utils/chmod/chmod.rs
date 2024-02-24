// https://man.archlinux.org/man/chmod.1

use std::error::Error;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn chmod(src: &str, mode: u32) -> Result<(), Box<dyn Error>> {
    fs::set_permissions(src, fs::Permissions::from_mode(mode))?;
    println!("Mode '{}' set on '{}'", mode, src);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = clop::get_opts();

    if opts.scrap.len() < 2 {
        panic!("Usage: chmod [OPTION]... <MODE> <TARGET>...");
    }

    let mode = opts.scrap[0].parse()?;

    for arg in &opts.scrap[1..] {
        chmod(arg, mode)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_file_mode() {
        let _ = fs::File::create("a");

        assert!(chmod("a", 755).is_ok());

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_change_dir_mode() {
        let _ = fs::create_dir("b");

        assert!(chmod("b", 644).is_ok());

        let _ = fs::remove_dir("b");
    }
}