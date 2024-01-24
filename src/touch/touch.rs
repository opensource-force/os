// https://man.archlinux.org/man/touch.1.en

use std::fs;

fn main() {
    let opts = clop::get_opts();
    
    for arg in &opts.scrap {
        match fs::File::create(arg) {
            Ok(file) => {
                println!("Created {}", arg);
        
                let _ = file.set_len(0);
            },
            Err(e) => println!("Error creating {}: {}", arg, e)
        }
    }
}