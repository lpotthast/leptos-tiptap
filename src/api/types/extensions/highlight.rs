use serde::{Deserialize, Serialize};

/// Attributes for highlight commands.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapHighlightAttributes {
    /// Optional CSS color value.
    pub color: Option<String>,
}
