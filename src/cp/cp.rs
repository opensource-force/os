// https://man.archlinux.org/man/cp.1
// -r | copy directories

use std::fs;

fn main() {
    let opts = clop::get_opts();
    
    if let Some(dest_path) = opts.scrap.last() {
        for arg in &opts.scrap[..opts.scrap.len() - 1] {
            match fs::copy(arg, dest_path) {
                Ok(_) => println!("Copied {} to {}", arg, dest_path),
                Err(e) => println!("Error copying {}: {}", arg, e)
            }
        }
    }
}