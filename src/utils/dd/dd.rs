// https://man.archlinux.org/man/dd.1

use std::{
    fs,
    error::Error,
    io::{Seek,SeekFrom,Read,Write}
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    source: String,
    #[clap(required = true)]
    dest: String,

    #[clap(short, long, default_value = "512")]
    block_size: u64,
}

fn dd(args: Args) -> Result<(), Box<dyn Error>> {
    let mut src = fs::File::open(&args.source)?;
    let mut dst = fs::File::create(&args.dest)?;
    let block_size = args.block_size as usize;
    let skip = 0;
    let mut buf = vec![0u8; skip as usize];
    let mut bytes_written = 0;
    
    src.read_exact(&mut buf)?;
    buf.resize(block_size, 0);

    dst.seek(SeekFrom::Start(skip))?;

    while let Ok(n) = src.read(&mut buf) {
        if n == 0 { break; }

        dst.write_all(&buf[..n])?;
        bytes_written += n;
    }
    
    println!("{} bytes written to '{}' from '{}'", bytes_written, &args.dest, &args.source);

    Ok(())
}

fn main() {
    let args = Args::parse();

    dd(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file() {
        let _ = fs::File::create("a");
        let args = Args {
            source: "a".to_string(),
            dest: "b".to_string(),
            block_size: 1
        };
    
        assert!(dd(args).is_ok());
    
        let _ = fs::remove_file("a");
        let _ = fs::remove_file("b");
    }
}