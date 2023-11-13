mod env;
mod error;
mod functions;
mod io;
mod openai;

use error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Read the secret we will be using either from the environment or from the `.env` file.
    let secret = env::secret_key_from_env().or_else(env::secret_key_from_dotenv_file);
    let Some(ref secret) = secret else {
        return Err("expected env `OPENAI_SECRET` to be set".into());
    };

    // Pre-populate the conversation with the context prompt.
    let mut messages = openai::prepare();

    // If the program was invoked with arguments, use them as the first user input.
    let args = env::prompt_from_args();
    if !args.is_empty() {
        openai::record_user_input(&mut messages, &args);
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
            .complete(&messages)
            .await
            .map_err(|err| err.to_string())?;
        little_snake.stop();
        // Then show it.
        io::show_reply(&reply);
        // And record it.
        openai::record_reply(&mut messages, reply.clone());

        // If the model asked us to call a function, do so.
        if let Some(call) = reply.function_call {
            let result = functions::apply(&call.name, &call.arguments);
            openai::record_function_call_result(&mut messages, &call.name, &result);

            // Re-trigger the completion to let the model know how the function call went.
            continue;
        }

        // Once the model has replied, ask the user for input.
        let input = io::read_user_input();
        if input.is_empty() {
            break;
        }
        openai::record_user_input(&mut messages, &input);
    }

    Ok(())
}
