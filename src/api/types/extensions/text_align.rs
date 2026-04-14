use serde::{Deserialize, Serialize};

/// Text alignment values supported by the text align extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TiptapTextAlign {
    /// Left alignment.
    Left,
    /// Center alignment.
    Center,
    /// Right alignment.
    Right,
    /// Justified alignment.
    Justify,
}
