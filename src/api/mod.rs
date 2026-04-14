mod commands;
#[cfg(feature = "component")]
pub(crate) mod component;
mod content;
mod editor;
mod error;
mod extensions;
mod types;
mod use_tiptap_editor;

pub use editor::TiptapEditor;
pub use editor::{TiptapEditorHandle, TiptapEditorInstance};
pub use error::{TiptapEditorError, TiptapEditorReport, TiptapEditorResult};
pub use extensions::TiptapExtension;
pub use types::{
    TiptapAttributes, TiptapCodeBlockAttributes, TiptapContent, TiptapFocusOptions,
    TiptapFocusTarget, TiptapHeadingLevel, TiptapHighlightAttributes, TiptapImageResource,
    TiptapInsertContentOptions, TiptapLinkResource, TiptapListKind, TiptapMarkName,
    TiptapMarkOptions, TiptapNodeName, TiptapParseOptions, TiptapPositionOrRange, TiptapRange,
    TiptapSchemaTarget, TiptapSelectionState, TiptapSetContentOptions, TiptapSplitBlockOptions,
    TiptapTextAlign, TiptapToggleListOptions, TiptapWhitespaceMode, TiptapYoutubeVideoResource,
};
pub use use_tiptap_editor::{
    UseTiptapEditorAttrs, UseTiptapEditorInput, UseTiptapEditorProps, UseTiptapEditorReturn,
    use_tiptap_editor,
};
