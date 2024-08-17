//! Universal constants for the program.

use serde_json::json;

pub const CONTEXT_PROMPT: &str = "\
You are a command-line program that assists the user in querying and editing a codebase \
using a large language model. Your mission is to provide a conversational interface \
to perform tasks on the codebase. You have access to the repository through the file system. \
Use only the provided functions to navigate and manipulate the codebase.
The user will provide high-level instructions, and you will use your available functions \
to complete the tasks.

Ask for clarification when needed, and keep responses concise, typically under a paragraph.
Minimize explanations: you're an expert programmer talking to an expert programmer.
If unsure about an answer, request more information from the user.

When asked about a particular definition, first use the `q` (query) function to find the files \
which have that definition. Then, use the `F` (read file) function to read them in detail \
and make sense of their contents.

When asked about the whole codebase or cross-cutting concerns, \
start by identifying relevant files with the `q` (query) function.
Next, use the `F` (read file) function to understand their contents.
To learn the file hierarchy, use the `f` (list files) function.
To understand the overall structure, read the `README.md` and CI files.
They will give you a hint of the overall structure.

Spell out your intermediate thoughts for each folder visited, \
or whenever you see reasonable.

When trying to edit the files, just provide the patch, without citing the full source.
If the tree is dirty, ask user's permissions before doing any edits.

Remember, you've got this! Believe in your abilities and provide the best assistance possible.
";

/// List all the functions as a JSON schema understood by the model.
pub fn all_functions() -> serde_json::Value {
    json!([
        {
            "type": "function",
            "function": {
                "name": "q",
                "description": "query abstract syntax tree",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "relative path to the file to parse, or to a directory to parse all the files in"
                        }
                    },
                    "required": ["path"],
                },
            }
        },
        {
            "type": "function",
            "function": {
                "name": "f",
                "description": "list files",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "relative path to the directory to look into"
                        }
                    },
                    "required": ["path"],
                },
            }
        },
        {
            "type": "function",
            "function": {
                "name": "F",
                "description": "read file",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "relative path to the file to read"
                        }
                    },
                    "required": ["path"],
                },
            }
        },
        {
            "type": "function",
            "function": {
                "name": "g",
                "description": "show commits log",
                "parameters": {},
            }
        },
        {
            "type": "function",
            "function": {
                "name": "G",
                "description": "show certain commit in details",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "hash": {
                            "type": "string",
                            "description": "hash-like of the commit to show"
                        }
                    },
                    "required": ["hash"],
                },
            }
        },
        {
            "type": "function",
            "function": {
                "name": "p",
                "description": "patch a file",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "relative path to the file to read"
                        },
                        "patch": {
                            "type": "string",
                            "description": "path to apply"
                        }
                    },
                    "required": ["path", "patch"],
                },
            }
        },
        // {"name": "r", "description": "ask the user to run a shell command", "parameters": {
        //     "type": "object",
        //     "properties": { "command": { "type": "string", "description": "shell command to run" } },
        //     "required": ["command"],
        // }}
    ])
}
