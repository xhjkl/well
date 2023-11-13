use colored::Colorize;

use crate::openai::Completion;

mod throbber;
pub use throbber::start_throbber;

/// Show the model's reply to the user.
pub fn show_reply(reply: &Completion) {
    if let Some(ref call) = reply.function_call {
        let call_notch = "<<".bright_cyan().dimmed().bold();
        let call_name = call.name.cyan();
        let call_arguments = call.arguments.cyan();
        eprintln!("{} {}{}", call_notch, call_name, call_arguments);
    }

    if let Some(ref content) = reply.content {
        let reply_notch = "<<".bright_green().dimmed().bold();
        eprintln!("{} {}", reply_notch, content);
    }

    if reply.content.is_none() && reply.function_call.is_none() {
        let reply_notch = "<<".bright_green().dimmed().bold();
        let nothing = "...".dimmed();
        eprintln!("{} {}", reply_notch, nothing);
    }
    eprintln!();
}

/// Read user input from stdin
pub fn read_user_input() -> String {
    let user_notch = ">>".bright_yellow().dimmed().bold();
    eprint!("{} ", user_notch);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    eprintln!();
    input.trim().to_string()
}

/// Show an already read user input.
pub fn show_user_input(input: &str) {
    let user_notch = ">>".bright_yellow().dimmed().bold();
    eprintln!("{} {}", user_notch, input);
}
