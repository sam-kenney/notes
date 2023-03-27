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
    pub id: Option<String>,
    pub title: String,
    pub body: String,
    pub timestamp: Option<i64>,
}

impl Note {
    /// Create a new note.
    ///
    /// # Arguments
    ///
    /// * `title` - A summary for the note.
    /// * `body` - The note's main content.
    /// * `id` - A unique identifier. If not provided, a new one will be generated.
    /// * `timestamp` - When the note was created or last updated.
    ///                 If not provided, the current time will be used.
    pub fn new(title: String, body: String, id: Option<String>, timestamp: Option<i64>) -> Self {
        Self {
            id: Some(id.unwrap_or(Uuid::new_v4().to_string())),
            title: title,
            body: body,
            timestamp: Some(timestamp.unwrap_or(chrono::Utc::now().timestamp())),
        }
    }
}
