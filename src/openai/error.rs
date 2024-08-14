use thiserror::Error;

use super::ErrorDetails;

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("Bad HTTP header: {0:?}")]
    BadHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("While sending a request: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Sent this: {0}\n\nGot this: {1}\n\nWhile Parsing: {2}")]
    SchemaMismatch(String, String, String),

    #[error("Protocol error: {0:?}")]
    ProtocolError(ErrorDetails),

    #[error("No choices in the completion")]
    NoChoice,
}
