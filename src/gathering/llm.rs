use ollama_rs::{
   Ollama,
   generation::chat::{ChatMessage, ChatMessageResponseStream, request::ChatMessageRequest},
};

use serde::{Deserialize, Serialize};

const SYSPROMPT: &str = "You are a research agent, you will be asked various questions and must respond using your knowladge and by using the provided internet search functionality.";

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
   let ollama = Ollama::new("https://localhost", 11434);
   ollama
}

impl Into<ChatMessageRequest> for &Conversation {
   fn into(self) -> ChatMessageRequest {
      let mut messages = self.0.clone();
      messages.insert(0, ChatMessage::system(SYSPROMPT.to_string()));
      messages.insert(
         1,
         ChatMessage::tool("Hello, how are you doing?".to_string()),
      );
      ChatMessageRequest::new("llama3".to_string(), messages)
   }
}

impl Conversation {
   pub fn push(&mut self, message: ChatMessage) {
      self.0.push(message);
   }
}
