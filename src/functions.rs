//! Functions available to the model by the function calling api.
use serde_json::json;

mod common;

mod query_ast;
use query_ast::rpc::query_ast;

mod list_files;
use list_files::rpc::list_files;

mod read_file;
use read_file::rpc::read_file;

mod list_commits;
use list_commits::rpc::list_commits;

mod show_commit;
use show_commit::rpc::show_commit;

fn to_json<T, E>(result: Result<T, E>) -> serde_json::value::Value
where
    T: ToString,
    E: ToString,
{
    match result {
        Ok(value) => json!({ "result": value.to_string() }),
        Err(error) => json!({ "error": error.to_string() }),
    }
}

/// Apply a function call to the conversation.
pub fn apply(name: &str, arguments: &str) -> String {
    let result = match name {
        "q" => query_ast(arguments),
        "f" => list_files(arguments),
        "F" => read_file(arguments),
        "g" => list_commits(arguments),
        "G" => show_commit(arguments),
        _ => Err(format!("no such function: `{name}`")),
    };

    to_json(result).to_string()
}
