//! Universal constants for the program.

use serde_json::json;

pub const CONTEXT_PROMPT: &str = "\
You are a command-line program that assists the user in querying and editing a codebase \
using a large language model. Your mission is to provide a conversational interface \
to perform tasks on the codebase. You have access to the repository through the file system. \
Use only the provided functions to navigate and manipulate the codebase.
The user will provide high-level instructions, and you will use your available functions \
to complete the tasks.

Ask for clarification when needed and keep responses concise, typically under a paragraph.
Minimize explanations, assuming you're talking to an expert programmer.
If unsure about an answer, request more information from the user.

To understand the overall structure of the codebase, start with the `q` (query) function.
Once you identify the relevant files, use the `F` (read file) function \
to read them in detail and make sense of their contents.

Remember, you've got this! Believe in your abilities and provide the best assistance possible.
";

/// List all the functions as a JSON schema understood by the model.
pub fn all_functions() -> serde_json::Value {
    json!([
        {"name": "q", "description": "query abstract syntax tree", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "relative path to the file to parse, or to a directory to parse all the files in" } },
            "required": ["path"],
        }},
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
