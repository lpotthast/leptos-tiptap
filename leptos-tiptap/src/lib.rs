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
    pub h1: bool,
    pub h2: bool,
    pub h3: bool,
    pub h4: bool,
    pub h5: bool,
    pub h6: bool,
    pub paragraph: bool,
    pub bold: bool,
    pub italic: bool,
    pub strike: bool,
    pub blockquote: bool,
    pub highlight: bool,
    pub align_left: bool,
    pub align_center: bool,
    pub align_right: bool,
    pub align_justify: bool,
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
    // Example: image.png
    pub title: String,
    // Example: "An example image, ..."
    pub alt: String,
    // Example: https:://my-site.com/public/image.png
    pub url: String,
}
