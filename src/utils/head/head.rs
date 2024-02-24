// https://man.archlinux.org/man/head.1.en

use std::io;
use std::io::BufRead;
use std::fs;

fn head(src: Option<&str>, lines: Option<u32>) -> io::Result<()> {
    if src.is_none() {
        let stdin = io::stdin();

        loop {
            let mut buf = String::new();
            
            stdin.read_line(&mut buf)?;
            println!("{}", buf);
        }
    }

    let file = fs::File::open(src.unwrap())?;
    let reader = io::BufReader::new(file);
    let line_count = lines.unwrap_or(0);
    let mut lines_read = 0;

    
    for line in reader.lines() {
        if line_count != 0 && lines_read >= line_count {
            break;
        }

        let content = line?;
        println!("{}", content);
        lines_read += 1;
    }

    Ok(())
}

fn main() {
    let mut opts = clop::get_opts();
    let some_lines = opts.has(&["n", "lines"], true).ok().and_then(|a| a.parse().ok());

    if opts.scrap.is_empty() {
        let _ = head(None, None);
        return
    }

    for arg in &opts.scrap {
        let _ = head(Some(arg), some_lines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_file_start() {
        let _ = fs::File::create("a");
        let _ = fs::write("a", "Hello\nWorld!");

        assert!(head(Some("a"), Some(1)).is_ok());

        let _ = fs::remove_file("a");
    }
}