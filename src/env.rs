use std::env;

/// Secret key from `OPENAI_SECRET_KEY`, if set.
pub fn secret_key_from_env() -> Option<String> {
    None.or_else(|| env::var("WELL_OPENAI_SECRET_KEY").ok())
        .or_else(|| env::var("WELL_OPENAI_API_KEY").ok())
        .or_else(|| env::var("OPENAI_SECRET_KEY").ok())
        .or_else(|| env::var("OPENAI_API_KEY").ok())
}

/// Model name from `OPENAI_MODEL`, if set.
pub fn model_name_from_env() -> Option<String> {
    None.or_else(|| env::var("WELL_OPENAI_MODEL").ok())
        .or_else(|| env::var("OPENAI_MODEL").ok())
}

/// Get from env `(secret, model_name)`, lazily loading `.env`.
pub fn vars() -> (Option<String>, Option<String>) {
    let secret = secret_key_from_env();
    let model_name = model_name_from_env();
    match (secret.as_ref(), model_name.as_ref()) {
        (Some(_), Some(_)) => {
            // If both are set, just return them.
            (secret, model_name)
        }
        _ => {
            // If some are missing, load `.env`, and propagate the values.
            dotenvy::dotenv().ok();

            // And then try again.
            (
                secret.or_else(secret_key_from_env),
                model_name.or_else(model_name_from_env),
            )
        }
    }
}

/// Build a prompt from `argv`
pub fn prompt_from_args() -> String {
    let mut prompt = String::new();
    for arg in std::env::args().skip(1) {
        prompt.push_str(&arg);
        prompt.push(' ');
    }
    prompt.trim().to_string()
}
