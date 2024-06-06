use std::error::Error;

use serde_json::json;

pub mod prompts;
pub mod schema;

pub use self::prompts::*;
pub use self::schema::*;

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
    pub async fn complete(
        &self,
        model: &str,
        messages: &[Message],
    ) -> Result<Completion, Box<dyn Error>> {
        let functions = all_functions();
        let completion: CompletionResponse = self
            .call(
                "chat/completions",
                &json!({
                    "model": model,
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

/// Convenience extensions for a `Vec<Message>`.
pub trait VecOfMessages {
    /// Make an empty conversation with the given context prompt.
    fn new_with_context(context: &str) -> Self;

    /// Add a message from the user to the conversation.
    fn push_user_message(&mut self, message: &str);

    /// Add a reply from the assistant to the conversation.
    fn push_assistant_message(&mut self, reply: Completion);

    /// Tell the assistant how a function call went.
    fn push_function_call_result(&mut self, function_name: &str, result: &str);
}

impl VecOfMessages for Vec<Message> {
    fn new_with_context(context: &str) -> Self {
        vec![Message {
            role: MessageRole::System,
            name: None,
            content: Some(context.to_string()),
            function_call: None,
        }]
    }

    fn push_user_message(&mut self, inquiry: &str) {
        self.push(Message {
            role: MessageRole::User,
            name: None,
            content: Some(inquiry.to_string()),
            function_call: None,
        });
    }

    fn push_assistant_message(&mut self, reply: Completion) {
        self.push(Message {
            role: MessageRole::Assistant,
            name: None,
            content: reply.content,
            function_call: reply.function_call,
        });
    }

    fn push_function_call_result(&mut self, function_name: &str, result: &str) {
        self.push(Message {
            role: MessageRole::Function,
            name: Some(function_name.to_string()),
            content: Some(result.to_string()),
            function_call: None,
        });
    }
}
