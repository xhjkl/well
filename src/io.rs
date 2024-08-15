use colored::Colorize;

use crate::openai::ToolCallRequest;

mod throbber;
pub use throbber::start_throbber;

/// Show the model's reply to the user.
pub fn show_reply(content: &str, tool_calls: &[ToolCallRequest]) {
    for call in tool_calls {
        let call_notch = "<<".bright_cyan().dimmed().bold();
        let call_name = call.function.name.cyan();
        let call_arguments = call.function.arguments.cyan();
        eprintln!("{} {}{}", call_notch, call_name, call_arguments);
    }

    if !content.is_empty() {
        let reply_notch = "<<".bright_green().dimmed().bold();
        eprintln!("{} {}", reply_notch, content);
    }

    if content.is_empty() && tool_calls.is_empty() {
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

/// Indicate the history has been altered.
pub fn show_history_alteration() {
    let notch = "==".bright_red().dimmed().bold();
    eprintln!("{}", notch);
}

/// Tell the user how well the history alteration worked.
pub fn show_history_altered(before: usize, after: usize) {
    let notch = "==".bright_red().dimmed().bold();
    let arrow = "->".dimmed().bold();
    let before = before.to_string().bright_red().dimmed().bold();
    let after = after.to_string().bright_red().dimmed().bold();
    eprintln!("{before} {arrow} {after}\n{notch}\n");
}
