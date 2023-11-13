use std::env;

/// Try to read `OPENAI_SECRET_KEY` from environment variables.
pub fn secret_key_from_env() -> Option<String> {
    None.or_else(|| env::var("WELL_OPENAI_SECRET_KEY").ok())
        .or_else(|| env::var("WELL_OPENAI_SECRET").ok())
        .or_else(|| env::var("OPENAI_SECRET_KEY").ok())
        .or_else(|| env::var("OPENAI_SECRET").ok())
}

/// Read `.env` and invoke `secret_key_from_env`.
pub fn secret_key_from_dotenv_file() -> Option<String> {
    dotenvy::dotenv().ok();

    secret_key_from_env()
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
