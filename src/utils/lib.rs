pub mod files {
    use std::error::Error;
    use std::fs;
    use std::io::{self, BufRead};

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
}