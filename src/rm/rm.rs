// https://man.archlinux.org/man/rm.1.en
// -r | remove_directory

use std::fs;

fn main() {
    let opts = clop::get_opts();

    for arg in &opts.scrap {
        match fs::remove_file(arg) {
            Ok(_) => println!("Removed {}", arg),
            Err(e) => println!("Error removing {}: {}", arg, e)
        }
    }
}