use serde::{Deserialize, Serialize};

/// Inclusive editor document range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TiptapRange {
    /// Start document position.
    pub from: u32,
    /// End document position.
    pub to: u32,
}

/// A single editor document position or a document range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TiptapPositionOrRange {
    /// A single document position.
    Position(u32),
    /// A document range.
    Range(TiptapRange),
}

impl From<u32> for TiptapPositionOrRange {
    fn from(value: u32) -> Self {
        Self::Position(value)
    }
}

impl From<TiptapRange> for TiptapPositionOrRange {
    fn from(value: TiptapRange) -> Self {
        Self::Range(value)
    }
}

/// Focus command target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapFocusTarget {
    /// Preserve the current focus target.
    Current,
    /// Focus the start of the document.
    Start,
    /// Focus the end of the document.
    End,
    /// Select all document content.
    All,
    /// Focus a concrete document position.
    At(u32),
}

/// Options for focusing the editor.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapFocusOptions {
    /// Whether the focused position should be scrolled into view.
    pub scroll_into_view: Option<bool>,
}

/// Options for mark commands.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapMarkOptions {
    /// Whether empty mark ranges should be extended.
    pub extend_empty_mark_range: Option<bool>,
}

/// Options for splitting a block.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapSplitBlockOptions {
    /// Whether active marks should be kept on the new block.
    pub keep_marks: Option<bool>,
}
