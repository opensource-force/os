use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;

fn main() {
    let mut opts = clop::get_opts();
    let is_symbolic = opts.has(&["s", "symbolic"], None);
    let og: &String;
    let link: String;

    if opts.scrap.len() == 2 {
        og = &opts.scrap[0];
        link = opts.scrap[1].to_string();
    }
    else if opts.scrap.len() == 1 {
        og = &opts.scrap[0];
        let filename = Path::new(og).file_name().unwrap().to_string_lossy();
        link = format!("./{}", filename);
    } else {
        println!("Incorrect number of arguments");
        return;
    }

    if is_symbolic {
        match unix_fs::symlink(og, &link) {
            Ok(_) => println!("Created symlink {}", link),
            Err(e) => println!("Error creating symlink {}", e)
        }
    } else {
        match fs::hard_link(og, &link) {
            Ok(_) => println!("Created hardlink {}", link),
            Err(e) => println!("Error creating hardlink {}", e)
        }
    }
}
