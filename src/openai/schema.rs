#![allow(dead_code)] // keeping schema fields at reach, even if not used

use monostate::MustBe;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Clone, Deserialize, Debug)]
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
    #[serde(rename = "tool_calls")]
    Call,
}

/// Who authored the message.
#[derive(Eq, Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// A message in the conversation.
#[derive(Eq, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Message {
    pub role: MessageRole,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            role: MessageRole::System,
            content: None,
            tool_call_id: None,
            tool_calls: None,
            refusal: None,
        }
    }
}

/// A function call request.
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct ToolCallRequestFunction {
    pub name: String,
    pub arguments: String,
}

/// A function call request.
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct ToolCallRequest {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: MustBe!("function"),
    pub function: ToolCallRequestFunction,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionToolParameters {
    /// Always `object`.
    #[serde(rename = "type")]
    pub kind: String,
    pub properties: serde_json::Value,
    pub required: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionToolDetails {
    /// Parsable name of the function, like `get_current_weather`.
    pub name: String,
    /// Natural-language description of the function,
    /// like `get the current weather in a given location`.
    pub description: String,
    /// The parameters the function takes.
    pub parameters: FunctionToolParameters,
}

/// What the model can do.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub kind: MustBe!("function"),
    pub function: FunctionToolDetails,
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
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
    pub system_fingerprint: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ErroneousCompletionResponse {
    pub error: ErrorDetails,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CompletionResponse {
    Success(SuccessfulCompletionResponse),
    Failure(ErroneousCompletionResponse),
}
