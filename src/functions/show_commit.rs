pub fn show_commit_with_hash(_hash: &str) -> Result<String, String> {
    Err("not today".to_string())
}

pub mod rpc {
    use super::*;

    pub fn show_commit(arguments: &str) -> Result<String, String> {
        #[derive(serde::Deserialize)]
        struct Arguments {
            hash: String,
        }
        let Arguments { hash } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

        show_commit_with_hash(&hash).map_err(|err| err.to_string())
    }
}
