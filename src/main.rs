mod env;
mod error;
mod functions;
mod io;
mod openai;

use error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    use openai::VecOfMessages;

    // Read the secret we will be using either from the environment or from the `.env` file.
    let (api_base, model, secret) = env::vars();
    let Some(ref secret) = secret else {
        return Err("expected env `OPENAI_SECRET` to be set".into());
    };
    let model = model.as_deref().unwrap_or("gpt-4o");

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
    let chat = openai::Chat::new(secret).map_err(|err| err.to_string())?;
    loop {
        // Generate the next message in the conversation.
        let little_snake = io::start_throbber();
        let reply = chat
            .complete(model, &messages)
            .await
            .map_err(|err| err.to_string())?;
        little_snake.stop();
        // And record it.
        messages.push_assistant_message(reply.clone());
        // And then show it.
        io::show_reply(&reply);

        // If the model asked us to call a function, do so.
        if let Some(call) = reply.function_call {
            let result = functions::apply(&call.name, &call.arguments);
            messages.push_function_call_result(&call.name, &result);

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
