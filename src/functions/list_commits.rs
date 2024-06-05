/* spell-checker:words chrono revwalk */

pub fn list_commits_in_current_repo() -> Result<String, String> {
    let repo = git2::Repository::discover(".").map_err(|err| err.to_string())?;

    let mut result = String::new();
    let mut revwalk = repo.revwalk().map_err(|err| err.to_string())?;
    revwalk.push_head().map_err(|err| err.to_string())?;
    for rev in revwalk {
        let rev = rev.map_err(|err| err.to_string())?;
        let commit = repo.find_commit(rev).map_err(|err| err.to_string())?;
        let date = chrono::DateTime::from_timestamp(commit.time().seconds(), 0)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default();
        let hash = commit.id().to_string();
        let summary = commit.summary().unwrap_or_default();
        let author = commit.author();
        let author = author.name().unwrap_or_default();

        result.push_str(&format!("[{date}] [{hash}] | {summary} {{{author}}}\n"));
    }

    Ok(result)
}

pub mod rpc {
    use super::*;

    /// `git log`
    pub fn list_commits(_arguments: &str) -> Result<String, String> {
        list_commits_in_current_repo()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "run manually to see output"]
    fn commits_listing_format() {
        let result = list_commits_in_current_repo().unwrap();
        println!("{}", result);
        assert!(false);
    }
}
