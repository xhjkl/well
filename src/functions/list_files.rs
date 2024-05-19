use std::{
    fs::DirEntry,
    io,
    os::unix::prelude::{MetadataExt, PermissionsExt},
    path::Path,
};

use super::common::path_spills_up;

/// `0b111` -> `rwx`.
fn rwx(perms: u32) -> String {
    let mut result = String::with_capacity(16);
    for shift in (0..9).rev() {
        if perms & (1 << shift) != 0 {
            match shift % 3 {
                2 => result.push('r'),
                1 => result.push('w'),
                0 => result.push('x'),
                _ => unreachable!(),
            }
        } else {
            result.push('-');
        }
    }
    result
}

/// One row of `ls`.
fn describe_entry(entry: DirEntry) -> std::io::Result<String> {
    let metadata = entry.metadata()?;
    let permissions = metadata.permissions();
    let file_type = metadata.file_type();
    let file_size = metadata.len();
    let mtime = metadata.modified()?.elapsed().unwrap().as_secs();
    let nlink = metadata.nlink();

    let mode = format!(
        "{}{}",
        if file_type.is_dir() {
            "d"
        } else if file_type.is_symlink() {
            "l"
        } else {
            "-"
        },
        rwx(permissions.mode()),
    );

    Ok(format!(
        "{mode} {nlink:4} {size:>8}B {mtime:>8} {name}",
        mode = mode,
        nlink = nlink,
        size = file_size,
        mtime = mtime,
        name = entry.file_name().to_string_lossy(),
    ))
}

/// `list_files` sans type conversion.
fn list_files_with_path(path: &Path) -> std::io::Result<String> {
    if path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "only paths relative to the current directory are available to list",
        ));
    }
    if path_spills_up(path) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "cannot list files outside the current directory",
        ));
    }
    let mut result = String::new();
    for entry in path.read_dir()? {
        let entry = entry?;
        result.push_str(&describe_entry(entry)?);
        result.push('\n');
    }
    Ok(result)
}

pub mod rpc {
    use super::*;

    /// `ls`
    pub fn list_files(arguments: &str) -> Result<String, String> {
        #[derive(serde::Deserialize)]
        struct Arguments {
            path: String,
        }
        let Arguments { path } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

        list_files_with_path(Path::new(&path)).map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "run manually to see output"]
    fn directory_listing_format() {
        let output = list_files_with_path(Path::new(".")).unwrap();
        eprintln!("{}", output);
        assert!(false);
    }
}
