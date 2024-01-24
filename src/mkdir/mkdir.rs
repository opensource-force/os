// https://man.archlinux.org/man/mkdir.1
// -p | make parent directories

use std::fs;

fn main() {
    let opts = clop::get_opts();

    for arg in &opts.scrap {
        match fs::create_dir(arg) {
            Ok(_) => println!("Created {}", arg),
            Err(e) => println!("Error creating {}: {}", arg, e)
        }
    }
}