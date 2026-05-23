use ollama_rs::{
   Ollama,
   generation::chat::{ChatMessage, ChatMessageResponseStream, request::ChatMessageRequest},
   generation::tools::ToolCall,
};

use serde::{Deserialize, Serialize};

const SYSPROMPT: &str = "You are a research agent. When you need to search the web, you should call the 'search' tool with a 'query' parameter containing your search query.";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation(pub Vec<ChatMessage>);

pub async fn stream_response(
   ollama: &Ollama,
   conversation: &Conversation,
) -> ChatMessageResponseStream {
   assert!(conversation.0.len() > 0);

   let request = conversation.into();
   ollama
      .send_chat_messages_stream(request)
      .await
      .expect("Failed to send chat message stream")
}

/// Helper function to create the Ollama instance
pub fn setup_ollama() -> Ollama {
   let ollama = Ollama::new("http://localhost", 11434);
   ollama
}

impl Into<ChatMessageRequest> for &Conversation {
   fn into(self) -> ChatMessageRequest {
      let mut messages = self.0.clone();
      messages.insert(0, ChatMessage::system(SYSPROMPT.to_string()));
      ChatMessageRequest::new("llama3:latest".to_string(), messages)
   }
}

impl Conversation {
   pub fn push(&mut self, message: ChatMessage) {
      self.0.push(message);
   }
}

/// Check if a ChatMessage contains tool calls
pub fn has_tool_calls(message: &ChatMessage) -> bool {
   !message.tool_calls.is_empty()
}

/// Extract tool calls from a ChatMessage
pub fn get_tool_calls(message: &ChatMessage) -> Vec<ToolCall> {
   message.tool_calls.clone()
}

/// Create a tool result message for the search tool
pub fn create_search_result_message(results: String) -> ChatMessage {
   ChatMessage::tool(results)
}
