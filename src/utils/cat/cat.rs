// https://man.archlinux.org/man/cat.1

use std::io::{self, stdin};
use std::fs;

fn cat(src: Option<&str>) -> io::Result<()> {
    if src.is_none() {
        loop {
            let mut buf = String::new();
            let _ = stdin().read_line(&mut buf);
    
            println!("{}", buf);
        }
    }

    let content = fs::read_to_string(src.unwrap())?;
    println!("{}", content);

    Ok(())
}

fn main() {
    let opts = clop::get_opts();

    if opts.scrap.len() == 0 {
        let _ = cat(None);
    } else {
        for arg in &opts.scrap {
            let _ = cat(Some(arg));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_file() {
        let _ = fs::File::create("a");
        let _ = fs::write("a", "Hello World!");

        assert!(cat(Some("a")).is_ok());

        let _ = fs::remove_file("a");
    }
}