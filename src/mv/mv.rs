// https://man.archlinux.org/man/mv.1.en

use std::fs;
use std::path::Path;

fn move_file(src: &str, dst: &str) -> bool {
    if let Err(e) = fs::metadata(src) {
        eprintln!("Cannot stat '{}': {}", src, e);
        return false
    }

    let src_path = Path::new(src);
    let dst_path = Path::new(dst);
    let dest = if dst_path.is_dir() {
        dst_path.join(src)
    } else {
        dst_path.to_path_buf()
    };
    
    if let Err(e) = fs::rename(src_path, &dest) {
        eprintln!("Error moving '{}': {}", src, e);
        return false
    }

    println!("Moved '{}' to '{}'", src, dest.to_string_lossy());
    return true
}

fn main() {
    let opts = clop::get_opts();

    if let Some(dst) = opts.scrap.last() {
        for arg in &opts.scrap[..opts.scrap.len() - 1] {
            move_file(arg, dst);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_file() {
        let _ = fs::File::create("a");

        assert!(move_file("a", "b"));

        let _ = fs::remove_file("b");
    }

    #[test]
    fn test_move_directory() {
        let _ = fs::create_dir("c");

        assert!(move_file("c", "d"));

        let _ = fs::remove_dir("d");
    }

    #[test]
    fn test_move_to_destination() {
        let _ = fs::File::create("e");
        let _ = fs::create_dir("f");

        assert!(move_file("e", "f"));

        let _ = fs::remove_dir_all("f");
    }
}