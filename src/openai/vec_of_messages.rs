use super::{Completion, Message, MessageRole};

/// Convenience extensions for a `Vec<Message>`.
pub trait VecOfMessages {
    /// Make an empty conversation with the given context prompt.
    fn new_with_context(context: &str) -> Self;

    /// Add a message from the user to the conversation.
    fn push_user_message(&mut self, message: &str);

    /// Add a reply from the assistant to the conversation.
    fn push_assistant_message(&mut self, reply: Completion);

    /// Tell the assistant how a function call went.
    fn push_function_call_result(&mut self, function_name: &str, result: &str);
}

impl VecOfMessages for Vec<Message> {
    fn new_with_context(context: &str) -> Self {
        vec![Message {
            role: MessageRole::System,
            name: None,
            content: Some(context.to_string()),
            function_call: None,
        }]
    }

    fn push_user_message(&mut self, inquiry: &str) {
        self.push(Message {
            role: MessageRole::User,
            name: None,
            content: Some(inquiry.to_string()),
            function_call: None,
        });
    }

    fn push_assistant_message(&mut self, reply: Completion) {
        self.push(Message {
            role: MessageRole::Assistant,
            name: None,
            content: reply.content,
            function_call: reply.function_call,
        });
    }

    fn push_function_call_result(&mut self, function_name: &str, result: &str) {
        self.push(Message {
            role: MessageRole::Function,
            name: Some(function_name.to_string()),
            content: Some(result.to_string()),
            function_call: None,
        });
    }
}
