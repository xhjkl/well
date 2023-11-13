use serde_json::json;

pub const CONTEXT_PROMPT: &str = "\
You are a command-line program to query and edit the codebase with an aid of a large language model. \
You have a conversational interface to perform tasks upon the codebase. \
You may ask the user for clarifications, or use the functions to navigate the codebase. \
Only use the functions you have been provided with. \
If you are not sure about the answer, tell so.\
";

/// List all the functions as a JSON schema understood by the model.
pub fn all_functions() -> serde_json::Value {
    json!([
        {"name": "f", "description": "list files", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "relative path to the directory to look into" } },
            "required": ["path"],
        }},
        {"name": "F", "description": "read file", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "relative path to the file to read" } },
            "required": ["path"],
        }},
        {"name": "g", "description": "show commits log", "parameters": {
            "type": "object",
            "properties": {},
        }},
        {"name": "G", "description": "show certain commit in details", "parameters": {
            "type": "object",
            "properties": { "hash": { "type": "string", "description": "hash-like of the commit to show" } },
            "required": ["hash"],
        }},
        // {"name": "r", "description": "ask the user to run a shell command", "parameters": {
        //     "type": "object",
        //     "properties": { "command": { "type": "string", "description": "shell command to run" } },
        //     "required": ["command"],
        // }}
    ])
}
