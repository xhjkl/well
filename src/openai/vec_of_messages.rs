use super::{Message, MessageRole, ToolCallRequest};

/// Convenience extensions for a `Vec<Message>`.
pub trait VecOfMessages {
    /// Make an empty conversation with the given context prompt.
    fn new_with_context(context: &str) -> Self;

    /// Add a message from the user to the conversation.
    fn push_user_message(&mut self, message: &str);

    /// Add a reply from the assistant to the conversation.
    fn push_assistant_message(&mut self, reply: &str, calls: &[ToolCallRequest]);

    /// Tell the assistant how a function call went.
    fn push_function_call_result(&mut self, id: &str, result: &str);

    /// Make a conversation briefer by forgetting the earlier function call results.
    fn strip(&mut self);
}

impl VecOfMessages for Vec<Message> {
    fn new_with_context(context: &str) -> Self {
        vec![Message {
            role: MessageRole::System,
            content: Some(context.to_string()),
            ..Default::default()
        }]
    }

    fn push_user_message(&mut self, inquiry: &str) {
        self.push(Message {
            role: MessageRole::User,
            content: Some(inquiry.to_string()),
            ..Default::default()
        });
    }

    fn push_assistant_message(&mut self, reply: &str, calls: &[ToolCallRequest]) {
        self.push(Message {
            role: MessageRole::Assistant,
            content: Some(reply.to_string()),
            tool_calls: (!calls.is_empty()).then(|| calls.to_vec()),
            ..Default::default()
        });
    }

    fn push_function_call_result(&mut self, id: &str, result: &str) {
        self.push(Message {
            role: MessageRole::Tool,
            tool_call_id: Some(id.to_string()),
            content: Some(result.to_string()),
            ..Default::default()
        });
    }

    fn strip(&mut self) {
        let mut seen_non_tool = false;
        for message in self.iter_mut().rev() {
            // Keeping only the last chain of tool calls
            // so that the model will not get confused.
            if seen_non_tool && message.role == MessageRole::Tool {
                // Setting this to `null` does not pass validation
                // on the OpenAI side, so we set it to an empty string instead.
                message.content = Some(String::new());
            }
            if message.role != MessageRole::Tool {
                seen_non_tool = true;
            }
        }
    }
}
