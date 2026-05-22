mod llm;
mod searching;
mod storage;

pub use crate::gathering::llm::{Conversation, setup_ollama, stream_response};
pub use crate::gathering::storage::{STORAGE_PATH, generate_session_id, store_conversation};

use crate::gathering::storage::SessionId;

pub struct SessionInfo {
   pub session_id: SessionId,
}
