use std::path::Path;

pub fn read_file(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        path: String,
    }
    let Arguments { path } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    std::fs::read_to_string(Path::new(&path)).map_err(|err| err.to_string())
}
