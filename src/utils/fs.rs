use colored::*;

// TODO move to utils/fs
/// Copies a `src` directory (recursively) to a `dest` directory (does not need to exist) <br />
/// If the `dest` directory already exists, fails with error.
pub fn copy_dir(src: &std::path::Path, dest: &std::path::Path) -> anyhow::Result<()> {
    let exists = dest.exists();
    if exists {
        return Err(anyhow::anyhow!(
            "directory '{}' already exists!",
            format!("{}", dest.display()).blue()
        ));
    }

    for entry in walkdir::WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();
        let mut dest_path = std::path::PathBuf::from(dest);
        dest_path.push(path.strip_prefix(src)?);

        if path.is_dir() {
            std::fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)?
            }
            std::fs::copy(path, &dest_path)?;
        }
    }
    Ok(())
}
