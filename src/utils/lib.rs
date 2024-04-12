pub mod files {
    use std::{
        fs,
        path::Path,
        borrow::Cow,
        io::{self, BufRead},
        error::Error
    };

    pub fn uid_to_user(uid: u32) -> Result<String, Box<dyn Error>> {
        if let Ok(file) = fs::File::open("/etc/passwd") {
            let reader = io::BufReader::new(file);

            for line in reader.lines().filter_map(|l| l.ok()) {
                let fields: Vec<&str> = line.split(':').collect();
                
                if fields.len() > 2 {
                    if let Ok(parsed_uid) = fields[2].parse::<u32>() {
                        if parsed_uid == uid {
                            return Ok(fields[0].to_string())
                        }
                    }
                }
            }
        }

        return Err(format!("Username not found for UID '{}'", uid).into())
    }


    pub fn user_to_uid(username: &str) -> Result<u32, Box<dyn Error>> {
        if let Ok(file) = fs::File::open("/etc/passwd") {
            let reader = io::BufReader::new(file);

            for line in reader.lines().filter_map(|l| l.ok()) {
                let fields: Vec<&str> = line.split(':').collect();

                if fields.len() > 0 && fields[0] == username {
                    if let Ok(uid) = fields[2].parse::<u32>() {
                        return Ok(uid)
                    }
                }
            }
        }

        return Err(format!("UID not found for '{}'", username).into())
    }

    pub fn gid_to_group(gid: u32) -> Result<String, Box<dyn Error>> {
        if let Ok(file) = fs::File::open("/etc/group") {
            let reader = io::BufReader::new(file);

            for line in reader.lines().filter_map(|l| l.ok()) {
                let fields: Vec<&str> = line.split(':').collect();

                if fields.len() > 2 {
                    if let Ok(parsed_gid) = fields[2].parse::<u32>() {
                        if parsed_gid == gid {
                            return Ok(fields[0].to_string())
                        }
                    }
                }
            }
        }

        return Err(format!("Group not found for GID '{}'", gid).into())
    }

    pub fn group_to_gid(group: &str) -> Result<u32, Box<dyn Error>> {
        if let Ok(file) = fs::File::open("/etc/group") {
            let reader = io::BufReader::new(file);

            for line in reader.lines().filter_map(|l| l.ok()) {
                let fields: Vec<&str> = line.split(':').collect();

                if fields.len() > 0 && fields[0] == group {
                    if let Ok(gid) = fields[2].parse::<u32>() {
                        return Ok(gid)
                    }
                }
            }
        }

        return Err(format!("GID not found for group '{}'", group).into())
    }

    pub fn copy_file(
        src_path: impl AsRef<Path>,
        dst_path: impl AsRef<Path>
    ) -> Result<(), Box<dyn Error>> {
        let src_path = src_path.as_ref();
        let dst_path = dst_path.as_ref();
    
        fs::copy(src_path, dst_path)?;
    
        Ok(())
    }
    
    pub fn copy_directory(
        src_path: impl AsRef<Path>,
        dst_path: impl AsRef<Path>,
        is_recursive: bool
    ) -> Result<(), Box<dyn Error>> {
        let src_path = src_path.as_ref();
        let dst_path = dst_path.as_ref();
    
        fs::create_dir_all(&dst_path)?;
    
        for entry in fs::read_dir(&src_path)? {
            let entry_path = entry?.path();
    
            let new_dst = match entry_path.file_name() {
                Some(file_name) => Cow::from(dst_path.join(file_name)),
                None => Cow::from(dst_path)
            };
    
            if entry_path.is_file() {
                copy_file(entry_path, new_dst)?;
            } else if entry_path.is_dir() && is_recursive {
                copy_directory(entry_path, new_dst, is_recursive)?;
            }
        }
    
        Ok(())
    }    
}