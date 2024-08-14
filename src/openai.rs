use serde_json::json;

pub mod error;
pub mod prompts;
pub mod schema;

pub mod vec_of_messages;

pub use self::prompts::*;
pub use self::schema::*;
pub use self::vec_of_messages::VecOfMessages;

/// Result with the right error.
pub type Result<T> = std::result::Result<T, error::OpenAIError>;

/// An HTTP client to the OpenAI Chat Completions API.
/// It does not hold any persistent connections, each completion is a new request.
pub struct Chat {
    base: String,
    client: reqwest::Client,
}

impl Chat {
    /// Create a new client with the given auth.
    pub fn new(base: &str, access_token: Option<&str>) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(access_token) = access_token {
            let mut authorization =
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token))?;
            authorization.set_sensitive(true);
            headers.insert(reqwest::header::AUTHORIZATION, authorization);
        }
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
    async fn call<Body, Response>(&self, endpoint: &str, body: &Body) -> Result<Response>
    where
        Body: serde::Serialize + std::fmt::Debug,
        Response: for<'re> serde::Deserialize<'re>,
    {
        let base = &self.base;
        let address = format!("{base}/{endpoint}");

        let response = self
            .client
            .post(address)
            .json(body)
            .send()
            .await?
            .text()
            .await?;

        let response: Response = serde_json::from_str(&response).map_err(|err| {
            error::OpenAIError::SchemaMismatch(
                serde_json::to_string_pretty(&body).unwrap(),
                response,
                err.to_string(),
            )
        })?;

        Ok(response)
    }

    /// Infer the next message in the conversation.
    pub async fn complete(
        &self,
        model: &str,
        messages: &[Message],
        tools: &serde_json::Value,
    ) -> Result<CompletionChoice> {
        let completion: CompletionResponse = self
            .call(
                "chat/completions",
                &json!({
                    "model": model,
                    "messages": messages,
                    "tools": tools,
                }),
            )
            .await?;

        let choices = match completion {
            CompletionResponse::Success(SuccessfulCompletionResponse { choices, .. }) => choices,
            CompletionResponse::Failure(ErroneousCompletionResponse { error }) => {
                return Err(error::OpenAIError::ProtocolError(error));
            }
        };
        let choice = choices
            .iter()
            .find(|choice| choice.finish_reason != FinishReason::UsageExceeded)
            .or_else(|| choices.first());
        let Some(choice) = choice.cloned() else {
            return Err(error::OpenAIError::NoChoice);
        };
        Ok(choice)
    }
}
