use serde::{Deserialize, Serialize};

/// Attributes for code block commands.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapCodeBlockAttributes {
    /// Optional language name for syntax highlighting.
    pub language: Option<String>,
}
