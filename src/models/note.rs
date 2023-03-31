use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Struct representing a note.
///
/// # Attributes
///
/// * `id` - A unique identifier.
/// * `title` - A summary for the note.
/// * `body` - The note's main content.
/// * `timestamp` - When the note was created or last updated.
#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    #[serde(default = "generate_uuid")]
    pub id: String,
    pub title: String,
    pub body: String,
    #[serde(default = "generate_timestamp")]
    pub timestamp: i64,
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn generate_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}
