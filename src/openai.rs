use std::error::Error;

use serde_json::json;

pub mod prompts;
pub mod types;

use self::prompts::*;
use self::types::*;

/// What the model returns.
#[derive(Debug, Clone)]
pub struct Completion {
    pub content: Option<String>,
    pub function_call: Option<FunctionCallRequest>,
}

/// An active connection to a correspondence between several agents.
pub struct Chat {
    client: reqwest::Client,
}

impl Chat {
    /// Create a new client with the given auth.
    pub fn new(access_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        let mut authorization =
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token))?;
        authorization.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, authorization);
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self { client })
    }

    /// Generic call to any of OpenAI API endpoints.
    async fn call<Body, Response>(
        &self,
        endpoint: &str,
        body: &Body,
    ) -> Result<Response, Box<dyn std::error::Error>>
    where
        Body: serde::Serialize + std::fmt::Debug,
        Response: for<'re> serde::Deserialize<'re>,
    {
        let address = format!("https://api.openai.com/v1/{endpoint}");

        let response: Response = self
            .client
            .post(address)
            .json(body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Generate the next message in the conversation.
    pub async fn complete(&self, messages: &[Message]) -> Result<Completion, Box<dyn Error>> {
        let functions = all_functions();
        let completion: CompletionResponse = self
            .call(
                "chat/completions",
                &json!({
                    "model": "gpt-3.5-turbo",
                    "messages": messages,
                    "functions": functions,
                }),
            )
            .await?;

        let choice = completion
            .choices
            .iter()
            .find(|choice| choice.finish_reason != FinishReason::UsageExceeded)
            .or_else(|| completion.choices.first());
        let Some(choice) = choice else {
            return Err("💔".into());
        };
        let content = choice.message.content.clone();
        let function_call = match choice.finish_reason {
            FinishReason::Call => choice.message.function_call.clone(),
            _ => None,
        };
        Ok(Completion {
            content,
            function_call,
        })
    }
}

/// Generate the beginning of the conversation.
pub fn prepare() -> Vec<Message> {
    vec![Message {
        role: MessageRole::System,
        name: None,
        content: Some(CONTEXT_PROMPT.to_string()),
        function_call: None,
    }]
}

/// Push a user's message to the conversation history.
pub fn record_user_input(messages: &mut Vec<Message>, input: &str) {
    messages.push(Message {
        role: MessageRole::User,
        name: None,
        content: Some(input.to_string()),
        function_call: None,
    });
}

/// Push a reply from the model to the conversation history.
pub fn record_reply(messages: &mut Vec<Message>, reply: Completion) {
    messages.push(Message {
        role: MessageRole::Assistant,
        name: None,
        content: reply.content,
        function_call: reply.function_call,
    });
}

/// Push a function call to the conversation history.
pub fn record_function_call_result(messages: &mut Vec<Message>, function_name: &str, result: &str) {
    messages.push(Message {
        role: MessageRole::Function,
        name: Some(function_name.to_string()),
        content: Some(result.to_string()),
        function_call: None,
    });
}
