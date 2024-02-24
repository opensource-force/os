//https://man.archlinux.org/man/rmdir.1
// -p | remove parent directories

use std::error::Error;
use std::fs;

fn rmdir(src: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_dir(src)?;
    println!("Removed directory '{}'", src);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = clop::get_opts();

    if opts.scrap.len() < 1 {
        panic!("Usage: rmdir [OPTION]... <DIRECTORY>...");
    }

    for arg in &opts.scrap {
        rmdir(arg)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dir() {
        let _ = fs::create_dir("a");

        assert!(rmdir("a").is_ok());
    }
}