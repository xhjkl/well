use std::io;
use std::path::Path;

use super::common::path_spills_up;

/// `< path`
pub fn read_file_with_path(path: &Path) -> io::Result<String> {
    if path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "only paths relative to the current directory are available to read",
        ));
    }
    if path_spills_up(path) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "cannot read files outside the current directory",
        ));
    }

    std::fs::read_to_string(Path::new(&path))
}

pub mod rpc {
    use super::*;

    /// `< path`
    pub fn read_file(arguments: &str) -> Result<String, String> {
        #[derive(serde::Deserialize)]
        struct Arguments {
            path: String,
        }
        let Arguments { path } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

        read_file_with_path(Path::new(&path)).map_err(|err| err.to_string())
    }
}
