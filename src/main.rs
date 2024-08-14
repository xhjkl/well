mod env;
mod error;
mod functions;
mod io;
mod openai;

use error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    use openai::VecOfMessages as _;

    // Read the secret we will be using either from the environment or from the `.env` file.
    let (api_base, model, secret) = env::vars();
    if api_base.is_none() && secret.is_none() {
        return Err("expected env `OPENAI_API_KEY` to be available".into());
    }
    let api_base = api_base.as_deref().unwrap_or("https://api.openai.com/v1");
    let model = model.as_deref().unwrap_or("gpt-4o");
    let secret = secret.as_deref();

    // Pre-populate the conversation with the context prompt.
    let mut messages = Vec::<openai::Message>::new_with_context(openai::CONTEXT_PROMPT);

    // If the program was invoked with arguments, use them as the first user input.
    let args = env::prompt_from_args();
    if !args.is_empty() {
        messages.push_user_message(&args);
        eprintln!();
        io::show_user_input(&args);
    }

    // Maintain symmetry.
    eprintln!();

    // Converse until the user enters an empty line.
    let chat = openai::Chat::new(api_base, secret).map_err(|err| err.to_string())?;
    loop {
        // Generate the next message in the conversation.
        let little_snake = io::start_throbber();
        let reply = chat
            .complete(model, &messages, &openai::all_functions())
            .await
            .map_err(|err| err.to_string())?;
        little_snake.stop();

        // The model may reply to us with a text,
        // or it may ask us to do something through a tool call.
        // It also may indicate that the conversation is too large for it to handle.
        // Or, it may refuse to continue due to policy.
        // These cases may overlap, so we take the most conservative route.
        if let Some(refusal) = reply.message.refusal {
            return Err(refusal.into());
        }
        if reply.finish_reason == openai::FinishReason::UsageExceeded {
            return Err("usage exceeded".into());
        }
        let calls = reply.message.tool_calls.unwrap_or_default();
        let content = reply.message.content.unwrap_or_default();

        // Record the reply and the function call ids, if there are any.
        messages.push_assistant_message(&content, &calls);
        io::show_reply(&content, &calls);

        // If the model asked us to call a function, do so.
        if !calls.is_empty() {
            let result = functions::apply_all(&calls);
            for (id, result) in result {
                messages.push_function_call_result(&id, &result);
            }

            // Re-trigger the completion to let the model know how the function call went.
            continue;
        }

        // Once the model has replied, ask the user for input.
        let input = io::read_user_input();
        if input.is_empty() {
            break;
        }
        messages.push_user_message(&input);
    }

    Ok(())
}
