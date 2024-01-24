// https://man.archlinux.org/man/ls.1.en
// -a | show hidden files

use std::env;
use std::fs;

fn main() {
    let mut opts = clop::get_opts();

    if opts.scrap.len() == 0 {
        let home = env::var("HOME").unwrap();
        opts.scrap.push(home);
    }

    for arg in &opts.scrap {
        match fs::read_dir(arg) {
            Ok(paths) => {
                for path in paths {
                    if let Ok(file) = path.unwrap().path().strip_prefix(arg) {
                        let file_name = file.file_name().unwrap().to_string_lossy();
                      
                        if !file_name.starts_with(".") {
                            println!("{}", file.display());
                        }
                    }
                }
            },
            Err(e) => println!("Error reading {}: {}", arg, e)
        }
    }
}