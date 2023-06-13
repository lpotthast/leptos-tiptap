use serde::{Deserialize, Serialize};

mod js_tiptap;
mod tiptap_instance;

pub use tiptap_instance::TiptapInstance;
pub use tiptap_instance::TiptapInstanceMsg;

/// State of the current editor. Contains the selection state.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditorState {
    editable: bool,
    selection: SelectionState,
}

/// State of the current selection.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectionState {
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
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl From<HeadingLevel> for i32 {
    fn from(val: HeadingLevel) -> Self {
        match val {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }
}
