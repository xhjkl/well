use std::env;

/// API base from `OPENAI_API_BASE`, if set.
pub fn api_base_from_env() -> Option<String> {
    None.or_else(|| env::var("WELL_OPENAI_API_BASE").ok())
        .or_else(|| env::var("OPENAI_API_BASE").ok())
}

/// Model name from `OPENAI_MODEL`, if set.
pub fn model_name_from_env() -> Option<String> {
    None.or_else(|| env::var("WELL_OPENAI_MODEL").ok())
        .or_else(|| env::var("OPENAI_MODEL").ok())
}

/// Secret key from `OPENAI_API_KEY`, if set.
pub fn secret_key_from_env() -> Option<String> {
    None.or_else(|| env::var("WELL_OPENAI_SECRET").ok())
        .or_else(|| env::var("WELL_OPENAI_API_KEY").ok())
        .or_else(|| env::var("OPENAI_SECRET").ok())
        .or_else(|| env::var("OPENAI_API_KEY").ok())
}

/// Get from env `(api_base, model_name, secret)`, lazily loading `.env`.
pub fn vars() -> (Option<String>, Option<String>, Option<String>) {
    let api_base = api_base_from_env();
    let model_name = model_name_from_env();
    let secret = secret_key_from_env();
    match (api_base.as_ref(), model_name.as_ref(), secret.as_ref()) {
        (Some(_), Some(_), Some(_)) => {
            // If all are set, just return them.
            (api_base, model_name, secret)
        }
        _ => {
            // If some are missing, load `.env`, and propagate the values.
            dotenvy::dotenv().ok();

            // And then try again.
            (
                api_base.or_else(api_base_from_env),
                model_name.or_else(model_name_from_env),
                secret.or_else(secret_key_from_env),
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
