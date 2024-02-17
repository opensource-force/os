// https://man.archlinux.org/man/cat.1

use std::io;
use std::fs;

fn cat(file: Option<&str>) -> io::Result<()> {
    if file.is_none() {
        loop {
            let mut buffer = String::new();
            let _ = stdin.read_line(&mut buffer);
    
            println!("{}", buffer);
        }
    }

    let content = fs::read_to_string(file)?;
    println!("{}", content);

    Ok(())
}

fn main() {
    let opts = clop::get_opts();
    let stdin = io::stdin();

    if opts.scrap.len() == 0 {

    } else {
        for arg in &opts.scrap {
            let _ = cat(arg);
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

        assert!(cat("a").is_ok());

        let _ = fs::remove_file("a");
    }
}