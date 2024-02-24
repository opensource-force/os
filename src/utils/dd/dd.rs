// https://man.archlinux.org/man/dd.1

use std::error::Error;
use std::io::{Seek,SeekFrom,Read,Write};
use std::fs;

fn dd(src: &str, dst: &str, bs: Option<usize>) -> Result<(), Box<dyn Error>> {
    let mut i_file = fs::File::open(src)?;
    let mut o_file = fs::File::create(dst)?;
    let block_size = bs.unwrap_or(512);
    let skip = 0;
    let mut buf = vec![0u8; skip as usize];
    let mut bytes_written = 0;
    
    i_file.read_exact(&mut buf)?;
    buf.resize(block_size, 0);

    o_file.seek(SeekFrom::Start(skip))?;

    while let Ok(n) = i_file.read(&mut buf) {
        if n == 0 {
            break;
        }
        o_file.write_all(&buf[..n])?;
        bytes_written += n;
    }
    println!("{} bytes written to '{}' from '{}'", bytes_written, dst, src);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = clop::get_opts();
    let some_bs = opts.has(&["b", "block-size"], true).ok().and_then(|a| a.parse().ok());

    if opts.scrap.len() != 2 {
        panic!("Usage:  dd [OPTION]... <FILE> <DESTINATION>");
    }

    let src = &opts.scrap[0];
    let dst = &opts.scrap[1];

    dd(src, dst, some_bs)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file() {
        let _ = fs::File::create("a");

        assert!(dd("a", "b", Some(1)).is_ok());

        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }
}