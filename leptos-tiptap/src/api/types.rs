use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::{error::Error, fmt};

/// Editor content payload.
///
/// `Html` contains editor content as HTML.
/// `Json` contains the Tiptap/ProseMirror JSON document as structured data.
#[derive(Debug, PartialEq, Clone)]
pub enum TiptapContent {
    Html(String),
    Json(serde_json::Value),
}

impl TiptapContent {
    pub fn html(content: impl Into<String>) -> Self {
        Self::Html(content.into())
    }

    pub fn json(content: impl Into<serde_json::Value>) -> Self {
        Self::Json(content.into())
    }

    pub fn json_str(content: impl AsRef<str>) -> Result<Self, serde_json::Error> {
        serde_json::from_str(content.as_ref()).map(Self::Json)
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TiptapAttributes(pub(crate) Map<String, serde_json::Value>);

impl TiptapAttributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Option<serde_json::Value> {
        self.0.insert(key.into(), value.into())
    }
}

impl From<Map<String, serde_json::Value>> for TiptapAttributes {
    fn from(value: Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TiptapRange {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TiptapPositionOrRange {
    Position(u32),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapFocusTarget {
    Current,
    Start,
    End,
    All,
    At(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapFocusOptions {
    pub scroll_into_view: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapWhitespaceMode {
    Preserve,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapParseOptions {
    pub preserve_whitespace: Option<TiptapWhitespaceMode>,
    pub from: Option<u32>,
    pub to: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapSetContentOptions {
    pub emit_update: Option<bool>,
    pub parse_options: Option<TiptapParseOptions>,
    pub error_on_invalid_content: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapInsertContentOptions {
    pub parse_options: Option<TiptapParseOptions>,
    pub update_selection: Option<bool>,
    pub apply_input_rules: Option<bool>,
    pub apply_paste_rules: Option<bool>,
    pub error_on_invalid_content: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapMarkOptions {
    pub extend_empty_mark_range: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapSplitBlockOptions {
    pub keep_marks: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TiptapToggleListOptions {
    pub keep_marks: Option<bool>,
    pub attributes: Option<TiptapAttributes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapSchemaTarget {
    Node(TiptapNodeName),
    Mark(TiptapMarkName),
}

impl TiptapSchemaTarget {
    pub(crate) fn schema_name(self) -> &'static str {
        match self {
            Self::Node(node) => node.schema_name(),
            Self::Mark(mark) => mark.schema_name(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TiptapNodeName {
    #[cfg(feature = "blockquote")]
    #[serde(rename = "blockquote")]
    Blockquote,
    #[cfg(feature = "bullet_list")]
    #[serde(rename = "bulletList")]
    BulletList,
    #[cfg(feature = "code_block")]
    #[serde(rename = "codeBlock")]
    CodeBlock,
    #[cfg(feature = "document")]
    #[serde(rename = "doc")]
    Doc,
    #[cfg(feature = "hard_break")]
    #[serde(rename = "hardBreak")]
    HardBreak,
    #[cfg(feature = "heading")]
    #[serde(rename = "heading")]
    Heading,
    #[cfg(feature = "horizontal_rule")]
    #[serde(rename = "horizontalRule")]
    HorizontalRule,
    #[cfg(feature = "image")]
    #[serde(rename = "image")]
    Image,
    #[cfg(feature = "list_item")]
    #[serde(rename = "listItem")]
    ListItem,
    #[cfg(feature = "ordered_list")]
    #[serde(rename = "orderedList")]
    OrderedList,
    #[cfg(feature = "paragraph")]
    #[serde(rename = "paragraph")]
    Paragraph,
    #[cfg(feature = "text")]
    #[serde(rename = "text")]
    Text,
    #[cfg(feature = "youtube")]
    #[serde(rename = "youtube")]
    Youtube,
}

impl TiptapNodeName {
    pub(crate) fn schema_name(self) -> &'static str {
        match self {
            #[cfg(feature = "blockquote")]
            Self::Blockquote => "blockquote",
            #[cfg(feature = "bullet_list")]
            Self::BulletList => "bulletList",
            #[cfg(feature = "code_block")]
            Self::CodeBlock => "codeBlock",
            #[cfg(feature = "document")]
            Self::Doc => "doc",
            #[cfg(feature = "hard_break")]
            Self::HardBreak => "hardBreak",
            #[cfg(feature = "heading")]
            Self::Heading => "heading",
            #[cfg(feature = "horizontal_rule")]
            Self::HorizontalRule => "horizontalRule",
            #[cfg(feature = "image")]
            Self::Image => "image",
            #[cfg(feature = "list_item")]
            Self::ListItem => "listItem",
            #[cfg(feature = "ordered_list")]
            Self::OrderedList => "orderedList",
            #[cfg(feature = "paragraph")]
            Self::Paragraph => "paragraph",
            #[cfg(feature = "text")]
            Self::Text => "text",
            #[cfg(feature = "youtube")]
            Self::Youtube => "youtube",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TiptapMarkName {
    #[cfg(feature = "bold")]
    #[serde(rename = "bold")]
    Bold,
    #[cfg(feature = "code")]
    #[serde(rename = "code")]
    Code,
    #[cfg(feature = "highlight")]
    #[serde(rename = "highlight")]
    Highlight,
    #[cfg(feature = "italic")]
    #[serde(rename = "italic")]
    Italic,
    #[cfg(feature = "link")]
    #[serde(rename = "link")]
    Link,
    #[cfg(feature = "strike")]
    #[serde(rename = "strike")]
    Strike,
}

impl TiptapMarkName {
    pub(crate) fn schema_name(self) -> &'static str {
        match self {
            #[cfg(feature = "bold")]
            Self::Bold => "bold",
            #[cfg(feature = "code")]
            Self::Code => "code",
            #[cfg(feature = "highlight")]
            Self::Highlight => "highlight",
            #[cfg(feature = "italic")]
            Self::Italic => "italic",
            #[cfg(feature = "link")]
            Self::Link => "link",
            #[cfg(feature = "strike")]
            Self::Strike => "strike",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TiptapListKind {
    #[cfg(feature = "bullet_list")]
    #[serde(rename = "bulletList")]
    BulletList,
    #[cfg(feature = "ordered_list")]
    #[serde(rename = "orderedList")]
    OrderedList,
}

impl TiptapListKind {
    pub(crate) fn list_name(self) -> &'static str {
        match self {
            #[cfg(feature = "bullet_list")]
            Self::BulletList => "bulletList",
            #[cfg(feature = "ordered_list")]
            Self::OrderedList => "orderedList",
        }
    }

    pub(crate) fn item_name(self) -> &'static str {
        "listItem"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TiptapEditorError {
    EditorUnavailable,
    DuplicateEditorId(String),
    MountFailed(String),
    InvalidContent(String),
    InvalidJson(String),
    CommandRejected { operation: String, message: String },
    OperationFailed { operation: String, message: String },
    BridgeError(String),
}

impl fmt::Display for TiptapEditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EditorUnavailable => {
                write!(f, "the requested Tiptap editor instance is not available")
            }
            Self::DuplicateEditorId(err) => {
                write!(f, "duplicate Tiptap editor id: {err}")
            }
            Self::MountFailed(err) => write!(f, "could not mount the Tiptap editor: {err}"),
            Self::InvalidContent(err) => write!(f, "invalid editor content: {err}"),
            Self::InvalidJson(err) => write!(f, "could not deserialize Tiptap JSON: {err}"),
            Self::CommandRejected { operation, message } => {
                write!(f, "editor command '{operation}' was rejected: {message}")
            }
            Self::OperationFailed { operation, message } => {
                write!(f, "editor operation '{operation}' failed: {message}")
            }
            Self::BridgeError(err) => write!(f, "Tiptap bridge error: {err}"),
        }
    }
}

impl Error for TiptapEditorError {}

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
    pub bullet_list: bool,
    pub ordered_list: bool,
    pub align_left: bool,
    pub align_center: bool,
    pub align_right: bool,
    pub align_justify: bool,
    pub link: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TiptapTextAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapCodeBlockAttributes {
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapHighlightAttributes {
    pub color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapImageResource {
    pub src: String,
    pub alt: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapLinkResource {
    pub href: String,
    pub target: Option<String>,
    pub rel: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapYoutubeVideoResource {
    pub src: String,
    pub start: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;
    use serde_json::json;

    #[test]
    fn display_invalid_content_error() {
        assert_that!(TiptapEditorError::InvalidContent("bad json".to_owned()).to_string())
            .is_equal_to("invalid editor content: bad json".to_owned());
    }

    #[test]
    fn display_command_rejected_error() {
        assert_that!(
            TiptapEditorError::CommandRejected {
                operation: "toggle_bold".to_owned(),
                message: "selection required".to_owned(),
            }
            .to_string()
        )
        .is_equal_to("editor command 'toggle_bold' was rejected: selection required".to_owned());
    }

    #[test]
    fn display_operation_failed_error() {
        assert_that!(
            TiptapEditorError::OperationFailed {
                operation: "read_html".to_owned(),
                message: "editor crashed".to_owned(),
            }
            .to_string()
        )
        .is_equal_to("editor operation 'read_html' failed: editor crashed".to_owned());
    }

    #[cfg(feature = "document")]
    #[test]
    fn serializes_node_names_to_exact_schema_names() {
        assert_that!(serde_json::to_value(TiptapNodeName::Doc).unwrap()).is_equal_to(json!("doc"));
    }

    #[cfg(feature = "bold")]
    #[test]
    fn serializes_mark_names_to_exact_schema_names() {
        assert_that!(serde_json::to_value(TiptapMarkName::Bold).unwrap())
            .is_equal_to(json!("bold"));
    }
}
