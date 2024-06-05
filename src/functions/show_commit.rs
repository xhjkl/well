/* spell-checker:words chrono */

use chrono::DateTime;
use git2::{DiffFormat, DiffOptions, Oid, Repository};

pub fn show_commit_with_hash(hash: &str) -> Result<String, String> {
    let repo = Repository::discover(".").map_err(|err| err.to_string())?;

    let commit = repo
        .find_commit(Oid::from_str(hash).map_err(|err| err.to_string())?)
        .map_err(|err| err.to_string())?;

    let date = DateTime::from_timestamp(commit.time().seconds(), 0)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default();
    let hash = commit.id().to_string();
    let summary = commit.summary().unwrap_or_default();
    let author = commit.author();
    let author_name = author.name().unwrap_or_default();
    let author_email = author.email().unwrap_or_default();

    let mut result = String::new();
    result.push_str(&format!(
        "Commit: {}\nAuthor: {} <{}>\nDate: {}\n\n    {}\n\n",
        hash, author_name, author_email, date, summary
    ));

    let parent = commit.parent(0).map_err(|err| err.to_string())?;
    let mut diff_opts = DiffOptions::new();
    let diff = repo
        .diff_tree_to_tree(
            Some(&parent.tree().map_err(|err| err.to_string())?),
            Some(&commit.tree().map_err(|err| err.to_string())?),
            Some(&mut diff_opts),
        )
        .map_err(|err| err.to_string())?;

    let mut patch = String::new();
    diff.print(DiffFormat::Patch, |_, _, line| {
        let mut origin = String::new();
        if "+ -".contains(line.origin()) {
            origin.push(line.origin());
        }
        patch.push_str(&format!(
            "{}{}",
            origin,
            String::from_utf8_lossy(line.content())
        ));
        true
    })
    .map_err(|err| err.to_string())?;

    result.push_str(&patch);

    Ok(result)
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
