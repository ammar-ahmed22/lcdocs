use colored::*;
use std::io::{BufRead, Write};

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

/// Deletes the lines around the target line in a file
pub fn delete_lines_around<P: AsRef<std::path::Path>>(
    file_path: P,
    target_line: &str,
    before: Option<usize>,
    after: Option<usize>,
) -> anyhow::Result<()> {
    let before = before.unwrap_or(0);
    let after = after.unwrap_or(0);

    // Read file line by line into vector
    let file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
    let reader = std::io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    // Find the index of the target
    if let Some(target_index) = lines.iter().position(|line| line.contains(target_line)) {
        // Remove the lines before, after and at the target
        let start_idx = target_index.saturating_sub(before);
        let end_idx = (target_index + after).min(lines.len() - 1);
        lines.drain(start_idx..end_idx + 1);
    }

    // Write updated lines back to the file
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;
    for line in lines.iter() {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
