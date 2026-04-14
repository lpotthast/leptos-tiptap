use serde::{Deserialize, Serialize};

/// Link resource used by link commands.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapLinkResource {
    /// Link URL.
    pub href: String,
    /// Optional link target.
    pub target: Option<String>,
    /// Optional link relationship value.
    pub rel: Option<String>,
    /// Optional CSS class value.
    pub class: Option<String>,
}
