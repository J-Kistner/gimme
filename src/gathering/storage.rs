use std::path::PathBuf;

use crate::gathering::llm::Conversation;

pub const STORAGE_PATH: &str = "/tmp/gimme/";

use crate::gathering::SessionInfo;

pub struct SessionId(pub String);

pub fn store_conversation(session_info: &SessionInfo, conversation: &Conversation) {
   let conversation_json =
      serde_json::to_string(&conversation).expect("Failed to serialize conversation");

   let mut path = PathBuf::from(STORAGE_PATH);
   assert!(path.exists(), "Session path does not exist");
   path.push(&session_info.session_id.0);

   std::fs::write(path, conversation_json).expect("Failed to write conversation to file");
}

pub fn generate_session_id() -> SessionId {
   use std::collections::hash_map::DefaultHasher;
   use std::hash::{Hash, Hasher};
   use std::time::{SystemTime, UNIX_EPOCH};

   let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
   assert!(now > 0);

   let mut hasher = DefaultHasher::new();
   now.hash(&mut hasher);
   let hash = format!("{:x}", hasher.finish());
   let session_id = SessionId(hash);

   session_id
}
