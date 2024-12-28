use serde::{Deserialize, Serialize};

mod js_tiptap;
mod tiptap_instance;

pub use tiptap_instance::TiptapInstance;
pub use tiptap_instance::TiptapInstanceMsg;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum TiptapContent {
    Html(String),
    Json(String),
}

/// State of the current editor. Contains the selection state.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TiptapEditorState {
    editable: bool,
    selection: TiptapSelectionState,
}

/// State of the current selection.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TiptapSelectionState {
    /// 'true' if the cursor is in a h1.
    pub h1: bool,

    /// 'true' if the cursor is in a h2.
    pub h2: bool,

    /// 'true' if the cursor is in a h3.
    pub h3: bool,

    /// 'true' if the cursor is in a h4.
    pub h4: bool,

    /// 'true' if the cursor is in a h5.
    pub h5: bool,

    /// 'true' if the cursor is in a h6.
    pub h6: bool,

    /// 'true' if the cursor is in a paragraph.
    pub paragraph: bool,

    /// 'true' if the cursor is in a bold text segment.
    pub bold: bool,

    /// 'true' if the cursor is in an italic text segment.
    pub italic: bool,

    /// 'true' if the cursor is in a strikethrough text segment.
    pub strike: bool,

    /// 'true' if the cursor is in a blockquote.
    pub blockquote: bool,

    /// 'true' if the cursor is in a highlighted text segment.
    pub highlight: bool,

    /// 'true' if the cursor is in a left-aligned text segment.
    pub align_left: bool,

    /// 'true' if the cursor is in a center-aligned text segment.
    pub align_center: bool,

    /// 'true' if the cursor is in a right-aligned text segment.
    pub align_right: bool,

    /// 'true' if the cursor is in a justify-aligned text segment.
    pub align_justify: bool,

    /// 'true' if the cursor is in a link.
    pub link: bool,

    /// 'true' if the cursor is on en embedded YouTube video.
    pub youtube: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapHeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl From<TiptapHeadingLevel> for i32 {
    fn from(val: TiptapHeadingLevel) -> Self {
        match val {
            TiptapHeadingLevel::H1 => 1,
            TiptapHeadingLevel::H2 => 2,
            TiptapHeadingLevel::H3 => 3,
            TiptapHeadingLevel::H4 => 4,
            TiptapHeadingLevel::H5 => 5,
            TiptapHeadingLevel::H6 => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapImageResource {
    /// Example: image.png
    pub title: String,

    /// Example: "An example image, ..."
    pub alt: String,

    /// Example: https:://my-site.com/public/image.png
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapLinkResource {
    /// Example: https:://my-site.com
    pub href: String,

    /// Example: "_blank", specifies where to open the linked document
    pub target: String,

    /// Example: "alternate"
    pub rel: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapYoutubeVideoResource {
    /// Example: https://www.youtube.com/embed/dQw4w9WgXcQ?si=6LwJzVo1t8hpLywC
    pub src: String,

    /// Example: "0", specifies when to start the video
    pub start: String,

    /// Example: "640"
    pub width: String,

    /// Example: "480"
    pub height: String,
}
