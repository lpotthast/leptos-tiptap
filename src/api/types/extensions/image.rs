use serde::{Deserialize, Serialize};

/// Image resource inserted by image commands.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapImageResource {
    /// Image source URL.
    pub src: String,
    /// Optional alternate text.
    pub alt: Option<String>,
    /// Optional image title.
    pub title: Option<String>,
}
