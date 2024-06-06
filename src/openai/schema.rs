#![allow(dead_code)] // keeping schema fields at reach, even if not used

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub completion_tokens: usize,
    pub prompt_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Deserialize, Debug)]
pub struct CompletionChoice {
    pub index: usize,
    pub finish_reason: FinishReason,
    pub message: Message,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Done,
    #[serde(rename = "length")]
    UsageExceeded,
    #[serde(rename = "function_call")]
    Call,
}

/// Who authored the message.
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    Function,
    User,
    Assistant,
}

/// A message in the conversation.
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCallRequest>,
}

/// A function call request.
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct FunctionCallRequest {
    pub name: String,
    pub arguments: String,
}

#[derive(Deserialize, Debug)]
pub struct ErroneousCompletionResponse {
    pub error: ErrorDetails,
}

#[derive(Deserialize, Debug)]
pub struct ErrorDetails {
    pub code: String,
    pub message: Option<String>,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Deserialize, Debug)]
pub struct SuccessfulCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: usize,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<CompletionChoice>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CompletionResponse {
    Success(SuccessfulCompletionResponse),
    Failure(ErroneousCompletionResponse),
}
