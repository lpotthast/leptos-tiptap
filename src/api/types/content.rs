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

impl Default for TiptapContent {
    /// Empty HTML content. Useful with [`UseTiptapEditorInput::default`] when
    /// the initial document is set later through `TiptapEditorHandle::set_content`.
    fn default() -> Self {
        Self::Html(String::new())
    }
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
