// https://man.archlinux.org/man/touch.1.en

use std::error::Error;
use std::fs;

fn make_file(src: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create(src)?;
    let _ = file.set_len(0);
    
    println!("Created '{}'", src);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = clop::get_opts();
    
    if opts.scrap.len() < 1 {
        panic!("Usage: touch [OPTION]... <FILE>...");
    }

    for arg in &opts.scrap {
        make_file(arg)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_file() {
        assert!(make_file("a").is_ok());

        let _ = fs::remove_file("a");
    }

    #[test]
    fn test_update_timestamp() {
        let _ = fs::File::create("b");

        assert!(make_file("b").is_ok());

        let _ = fs::remove_file("b");
    }
}