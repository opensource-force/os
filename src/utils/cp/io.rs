use super::*;

pub fn copy_file(src_path: impl AsRef<Path>, dst_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let src_path = src_path.as_ref();
    let dst_path = dst_path.as_ref();

    fs::copy(src_path, dst_path)?;
    println!("Copied '{}' to '{}'", src_path.display(), dst_path.display());

    Ok(())
}

pub fn copy_directory(src_path: impl AsRef<Path>, dst_path: impl AsRef<Path>, is_recursive: bool) -> Result<(), Box<dyn Error>> {
    let src_path = src_path.as_ref();
    let dst_path = dst_path.as_ref();

    fs::create_dir_all(&dst_path)?;

    for entry in fs::read_dir(&src_path)? {
        let entry_path = entry?.path();

        let new_dst = match entry_path.file_name() {
            Some(file_name) => Cow::from(dst_path.join(file_name)),
            None => Cow::from(dst_path),  // this should never happen
        };

        if entry_path.is_file() {
            copy_file(entry_path, new_dst)?;
        } else if entry_path.is_dir() && is_recursive {
            copy_directory(entry_path, new_dst, is_recursive)?;
        } else {
            // invalid path (shouldn't happen as it's a result of fs::read_dir, but I am not sure)
            // My suggestion is to ignore this case and print a warning.
        }
    }

    Ok(())
}
