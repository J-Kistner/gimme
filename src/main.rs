mod gathering;

use crate::gathering::{
   Conversation, SessionInfo, create_search_result_message, generate_session_id, get_tool_calls,
   has_tool_calls, search, setup_ollama, store_conversation, stream_response,
};
use gathering::STORAGE_PATH;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::ChatMessageResponseStream;
use std::env::args;
use tokio;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
   match setup_storage_path() {
      Ok(_) => {}
      Err(e) => {
         eprintln!("Failed to setup gimme: {}", e);
         std::process::exit(1);
      }
   }

   let args = args().collect::<Vec<String>>();
   assert!(args.len() > 0);
   let query = args.into_iter().skip(1).collect::<Vec<String>>().join(" ");
   assert!(query.len() > 0);

   let inital_message = ChatMessage::user(query);
   let mut conversation = Conversation(vec![inital_message]);
   let ollama = setup_ollama();
   let session_id = generate_session_id();
   let session_info = SessionInfo {
      session_id: session_id,
   };

   // LLM Response/printing
   let complete_response = {
      let mut stream: ChatMessageResponseStream = stream_response(&ollama, &conversation).await;
      let mut complete_response = String::new();
      while let Some(response) = stream.next().await {
         let response = response.expect("Failed to get response");
         complete_response = complete_response + &response.message.content;
         print!("{}", response.message.content);
      }
      complete_response
   };
}

fn setup_storage_path() -> Result<(), Box<dyn std::error::Error>> {
   let path = std::path::Path::new(STORAGE_PATH);
   if !path.exists() {
      std::fs::create_dir_all(path)?;
   }
   Ok(())
}
