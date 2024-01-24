// https://man.archlinux.org/man/ln.1

use std::fs;
use std::os::unix::fs as unix_fs;

fn main() {
    let mut opts = clop::get_opts();
    
    if opts.scrap.len() == 2 {
        if opts.has(&["s", "symbolic"], None) {
            let og = &opts.scrap[0];
            let link = &opts.scrap[1];

            match unix_fs::symlink(og, link) {
                Ok(_) => println!("Created symlink {}", link),
                Err(e) => println!("Error creating symlink {}", e)
            }
        } else {
            let og = &opts.scrap[0];
            let link = &opts.scrap[1];
            
            match fs::hard_link(og, link) {
                Ok(_) => println!("Created hardlink {}", link),
                Err(e) => println!("Error creating hardlink {}", e)
            }
        }
    } else {
        println!("Incorrect number of arguments");
    }
}