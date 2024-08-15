#![allow(dead_code)]
//! Making the conversation shorter.

use serde_json::json;

pub const CONTEXT_PROMPT: &str = "\
You are a conversation summary machine.

Your task is to summarize a long OpenAI chat into a shorter, concise version \
while maintaining the original data structure. \
The output will be fed back into the inference engine, so it is crucial that:
 * the system prompt stays the same;
 * roles remain intact: the user stays the user, and the assistant stays the assistant;
 * the output must be a valid, parsable OpenAI chat.

For parts of the conversation that are a solved problem, reduce it to just two messages:
 * the user's initial query;
 * the assistant's final solution.

For ongoing problems: keep all function calls and relevant messages, but rephrase for brevity.

For the system prompt: retain it intact.

Prioritize as follows:
 * first, drop the longest messages;
 * next, drop the earliest messages;
 * shorten more aggressively closer to the beginning of the thread.

When in doubt, retain the original content.

Believe in your abilities and provide the best, most concise summary possible!
";

/// What we send to the model to adhere to.
/// This instructs the model to return something parsable as a chat.
pub fn response_format() -> serde_json::Value {
    json!({
        "type": "json_schema",
        "json_schema": {
            "name": "chat",
            "description": "An OpenAI-compatible chat",
            "strict": true,
            "schema": {
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object",
                "properties": {
                    "messages": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "role": {
                                    "type": "string",
                                    "enum": ["system", "user", "assistant", "tool"],
                                },
                                "content": {
                                    "type": ["string", "null"],
                                },
                                "tool_calls": {
                                    "type": "array",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "id": {
                                                "type": "string",
                                            },
                                            "type": {
                                                "type": "string",
                                            },
                                            "function": {
                                                "type": "object",
                                                "properties": {
                                                    "name": {
                                                        "type": "string",
                                                    },
                                                    "arguments": {
                                                        "type": "string",
                                                    }
                                                },
                                                "required": ["name", "arguments"],
                                                "additionalProperties": false,
                                            }
                                        },
                                        "required": ["id", "type", "function"],
                                        "additionalProperties": false,
                                    }
                                },
                                "tool_call_id": {
                                    "type": "string",
                                }
                            },
                            "required": ["role", "content"],
                            "additionalProperties": false,
                        },
                    },
                },
                "required": ["messages"],
                "additionalProperties": false,
            },
        }
    })
}
