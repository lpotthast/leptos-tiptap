use serde::{Deserialize, Serialize};

/// `YouTube` video resource inserted by `YouTube` commands.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapYoutubeVideoResource {
    /// `YouTube` video URL.
    pub src: String,
    /// Optional start offset in seconds.
    pub start: Option<u32>,
    /// Optional embed width.
    pub width: Option<u32>,
    /// Optional embed height.
    pub height: Option<u32>,
}
