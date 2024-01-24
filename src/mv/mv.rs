// https://man.archlinux.org/man/mv.1.en

use std::fs;

fn main() {
    let opts = clop::get_opts();
    
    if let Some(dest_path) = opts.scrap.last() {
        for arg in &opts.scrap[..opts.scrap.len() - 1] {
            match fs::rename(arg, dest_path) {
                Ok(_) => println!("Moved {} to {}", arg, dest_path),
                Err(e) => eprintln!("Error moving {}: {}", arg, e)
            }
        }
    }
}