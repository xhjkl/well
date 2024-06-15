use std::error::Error;

use serde_json::json;

pub mod prompts;
pub mod schema;

pub mod vec_of_messages;

pub use self::prompts::*;
pub use self::schema::*;
pub use self::vec_of_messages::VecOfMessages;

/// What the model returns.
/// At least one of the fields shall be set.
#[derive(Debug, Clone)]
pub struct Completion {
    pub content: Option<String>,
    pub function_call: Option<FunctionCallRequest>,
}

/// An HTTP client to the OpenAI Chat Completions API.
/// It does not hold any persistent connections, each completion is a new request.
pub struct Chat {
    base: String,
    client: reqwest::Client,
}

impl Chat {
    /// Create a new client with the given auth.
    pub fn new(base: &str, access_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
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
        let base = base.to_string();
        Ok(Self { base, client })
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
        let base = &self.base;
        let address = format!("{base}/{endpoint}");

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

    /// Infer the next message in the conversation.
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

        let choices = match completion {
            CompletionResponse::Success(SuccessfulCompletionResponse { choices, .. }) => choices,
            CompletionResponse::Failure(ErroneousCompletionResponse { error }) => {
                return Err(format!("Error from OpenAI: {error:?}").into());
            }
        };
        let choice = choices
            .iter()
            .find(|choice| choice.finish_reason != FinishReason::UsageExceeded)
            .or_else(|| choices.first());
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
