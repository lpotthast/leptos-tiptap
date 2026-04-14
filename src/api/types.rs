use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::{error::Error, fmt};

/// Editor content payload.
///
/// `Html` contains editor content as HTML.
/// `Json` contains the Tiptap/ProseMirror JSON document as structured data.
#[derive(Debug, PartialEq, Clone)]
pub enum TiptapContent {
    /// HTML content.
    Html(String),
    /// Tiptap/ProseMirror JSON document content.
    Json(serde_json::Value),
}

impl TiptapContent {
    /// Creates an HTML content payload.
    #[must_use]
    pub fn html(content: impl Into<String>) -> Self {
        Self::Html(content.into())
    }

    /// Creates a JSON content payload.
    #[must_use]
    pub fn json(content: impl Into<serde_json::Value>) -> Self {
        Self::Json(content.into())
    }

    /// Parses a JSON string into a content payload.
    ///
    /// # Errors
    ///
    /// Returns a serde error when the string is not valid JSON.
    pub fn json_str(content: impl AsRef<str>) -> Result<Self, serde_json::Error> {
        serde_json::from_str(content.as_ref()).map(Self::Json)
    }
}

/// Arbitrary Tiptap node or mark attributes.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TiptapAttributes(pub(crate) Map<String, serde_json::Value>);

impl TiptapAttributes {
    /// Creates an empty attribute map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an attribute value and returns the previous value, if present.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Option<serde_json::Value> {
        self.0.insert(key.into(), value.into())
    }

    /// Returns an attribute value by key.
    #[must_use]
    pub fn get(&self, key: impl AsRef<str>) -> Option<&serde_json::Value> {
        self.0.get(key.as_ref())
    }

    /// Returns the underlying attribute map.
    #[must_use]
    pub fn as_map(&self) -> &Map<String, serde_json::Value> {
        &self.0
    }

    /// Returns the underlying attribute map mutably.
    pub fn as_mut_map(&mut self) -> &mut Map<String, serde_json::Value> {
        &mut self.0
    }

    /// Consumes the attributes and returns the underlying map.
    #[must_use]
    pub fn into_map(self) -> Map<String, serde_json::Value> {
        self.0
    }
}

impl From<Map<String, serde_json::Value>> for TiptapAttributes {
    fn from(value: Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}

impl<K, V> FromIterator<(K, V)> for TiptapAttributes
where
    K: Into<String>,
    V: Into<serde_json::Value>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut attributes = Self::new();
        attributes.extend(iter);
        attributes
    }
}

impl<K, V> Extend<(K, V)> for TiptapAttributes
where
    K: Into<String>,
    V: Into<serde_json::Value>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(
            iter.into_iter()
                .map(|(key, value)| (key.into(), value.into())),
        );
    }
}

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

/// Whitespace handling mode for parsing content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapWhitespaceMode {
    /// Preserve whitespace using Tiptap's `preserve` mode.
    Preserve,
    /// Preserve whitespace using Tiptap's `full` mode.
    Full,
}

/// Options passed to Tiptap content parsing.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapParseOptions {
    /// Optional whitespace preservation mode.
    pub preserve_whitespace: Option<TiptapWhitespaceMode>,
    /// Optional start position for parsing.
    pub from: Option<u32>,
    /// Optional end position for parsing.
    pub to: Option<u32>,
}

/// Options for replacing the editor document content.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapSetContentOptions {
    /// Whether the replacement should emit an update event.
    pub emit_update: Option<bool>,
    /// Optional parse options for the new content.
    pub parse_options: Option<TiptapParseOptions>,
    /// Whether invalid content should be reported as an error.
    pub error_on_invalid_content: Option<bool>,
}

/// Options for inserting content.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TiptapInsertContentOptions {
    /// Optional parse options for the inserted content.
    pub parse_options: Option<TiptapParseOptions>,
    /// Whether insertion should update the current selection.
    pub update_selection: Option<bool>,
    /// Whether input rules should be applied.
    pub apply_input_rules: Option<bool>,
    /// Whether paste rules should be applied.
    pub apply_paste_rules: Option<bool>,
    /// Whether invalid content should be reported as an error.
    pub error_on_invalid_content: Option<bool>,
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

/// Options for toggling list nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TiptapToggleListOptions {
    /// Whether active marks should be kept on the new list item.
    pub keep_marks: Option<bool>,
    /// Optional attributes to apply to the list.
    pub attributes: Option<TiptapAttributes>,
}

/// A node or mark target for schema-based commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapSchemaTarget {
    /// A node type target.
    Node(TiptapNodeName),
    /// A mark type target.
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

/// Tiptap node schema names supported by enabled Cargo features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TiptapNodeName {
    #[cfg(feature = "blockquote")]
    #[serde(rename = "blockquote")]
    /// The `blockquote` node.
    Blockquote,
    #[cfg(feature = "bullet_list")]
    #[serde(rename = "bulletList")]
    /// The `bulletList` node.
    BulletList,
    #[cfg(feature = "code_block")]
    #[serde(rename = "codeBlock")]
    /// The `codeBlock` node.
    CodeBlock,
    #[cfg(feature = "document")]
    #[serde(rename = "doc")]
    /// The `doc` node.
    Doc,
    #[cfg(feature = "hard_break")]
    #[serde(rename = "hardBreak")]
    /// The `hardBreak` node.
    HardBreak,
    #[cfg(feature = "heading")]
    #[serde(rename = "heading")]
    /// The `heading` node.
    Heading,
    #[cfg(feature = "horizontal_rule")]
    #[serde(rename = "horizontalRule")]
    /// The `horizontalRule` node.
    HorizontalRule,
    #[cfg(feature = "image")]
    #[serde(rename = "image")]
    /// The `image` node.
    Image,
    #[cfg(feature = "list_item")]
    #[serde(rename = "listItem")]
    /// The `listItem` node.
    ListItem,
    #[cfg(feature = "ordered_list")]
    #[serde(rename = "orderedList")]
    /// The `orderedList` node.
    OrderedList,
    #[cfg(feature = "paragraph")]
    #[serde(rename = "paragraph")]
    /// The `paragraph` node.
    Paragraph,
    #[cfg(feature = "text")]
    #[serde(rename = "text")]
    /// The `text` node.
    Text,
    #[cfg(feature = "youtube")]
    #[serde(rename = "youtube")]
    /// The `youtube` node.
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

/// Tiptap mark schema names supported by enabled Cargo features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TiptapMarkName {
    #[cfg(feature = "bold")]
    #[serde(rename = "bold")]
    /// The `bold` mark.
    Bold,
    #[cfg(feature = "code")]
    #[serde(rename = "code")]
    /// The `code` mark.
    Code,
    #[cfg(feature = "highlight")]
    #[serde(rename = "highlight")]
    /// The `highlight` mark.
    Highlight,
    #[cfg(feature = "italic")]
    #[serde(rename = "italic")]
    /// The `italic` mark.
    Italic,
    #[cfg(feature = "link")]
    #[serde(rename = "link")]
    /// The `link` mark.
    Link,
    #[cfg(feature = "strike")]
    #[serde(rename = "strike")]
    /// The `strike` mark.
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

/// List node kind for list commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TiptapListKind {
    #[cfg(feature = "bullet_list")]
    #[serde(rename = "bulletList")]
    /// A bullet list.
    BulletList,
    #[cfg(feature = "ordered_list")]
    #[serde(rename = "orderedList")]
    /// An ordered list.
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

    pub(crate) fn item_name() -> &'static str {
        "listItem"
    }
}

/// Error type for editor operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TiptapEditorError {
    /// The requested editor instance is unavailable.
    EditorUnavailable,
    /// An editor id is already in use.
    DuplicateEditorId(String),
    /// The editor could not be mounted.
    MountFailed(String),
    /// The provided editor content is invalid.
    InvalidContent(String),
    /// A JSON payload could not be parsed.
    InvalidJson(String),
    /// A bridge payload could not be parsed.
    InvalidBridgePayload(String),
    /// A command was rejected by the editor.
    CommandRejected {
        /// The rejected operation name.
        operation: String,
        /// The rejection message.
        message: String,
    },
    /// An editor operation failed.
    OperationFailed {
        /// The failed operation name.
        operation: String,
        /// The failure message.
        message: String,
    },
    /// The browser bridge returned an error.
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
            Self::InvalidBridgePayload(err) => {
                write!(f, "could not deserialize Tiptap bridge payload: {err}")
            }
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
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TiptapSelectionState {
    /// Whether the selection is in a level-1 heading.
    pub h1: bool,
    /// Whether the selection is in a level-2 heading.
    pub h2: bool,
    /// Whether the selection is in a level-3 heading.
    pub h3: bool,
    /// Whether the selection is in a level-4 heading.
    pub h4: bool,
    /// Whether the selection is in a level-5 heading.
    pub h5: bool,
    /// Whether the selection is in a level-6 heading.
    pub h6: bool,
    /// Whether the selection is in a paragraph.
    pub paragraph: bool,
    /// Whether bold is active in the selection.
    pub bold: bool,
    /// Whether italic is active in the selection.
    pub italic: bool,
    /// Whether strike is active in the selection.
    pub strike: bool,
    /// Whether the selection is in a blockquote.
    pub blockquote: bool,
    /// Whether highlight is active in the selection.
    pub highlight: bool,
    /// Whether the selection is in a bullet list.
    pub bullet_list: bool,
    /// Whether the selection is in an ordered list.
    pub ordered_list: bool,
    /// Whether left alignment is active in the selection.
    pub align_left: bool,
    /// Whether center alignment is active in the selection.
    pub align_center: bool,
    /// Whether right alignment is active in the selection.
    pub align_right: bool,
    /// Whether justified alignment is active in the selection.
    pub align_justify: bool,
    /// Whether link is active in the selection.
    pub link: bool,
    /// Whether the selection is in a `YouTube` node.
    pub youtube: bool,
}

/// Heading levels supported by Tiptap's heading extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapHeadingLevel {
    /// Level-1 heading.
    H1,
    /// Level-2 heading.
    H2,
    /// Level-3 heading.
    H3,
    /// Level-4 heading.
    H4,
    /// Level-5 heading.
    H5,
    /// Level-6 heading.
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

/// Attributes for code block commands.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapCodeBlockAttributes {
    /// Optional language name for syntax highlighting.
    pub language: Option<String>,
}

/// Attributes for highlight commands.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapHighlightAttributes {
    /// Optional CSS color value.
    pub color: Option<String>,
}

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
    fn display_invalid_bridge_payload_error() {
        assert_that!(
            TiptapEditorError::InvalidBridgePayload("bad selection".to_owned()).to_string()
        )
        .is_equal_to("could not deserialize Tiptap bridge payload: bad selection".to_owned());
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

    #[test]
    fn attributes_support_map_accessors_and_collect() {
        let mut attributes = [
            ("href", serde_json::json!("https://example.com")),
            ("rel", serde_json::json!("noopener")),
        ]
        .into_iter()
        .collect::<TiptapAttributes>();

        assert_that!(attributes.get("href")).is_some();

        attributes
            .as_mut_map()
            .insert("target".to_owned(), serde_json::json!("_blank"));

        assert_that!(attributes.as_map().contains_key("target")).is_true();
        assert_that!(attributes.into_map().contains_key("rel")).is_true();
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
