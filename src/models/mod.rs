use crate::AppState;
use std::sync::Arc;

mod note;
pub use note::Note;

pub type SharedState = Arc<AppState>;
