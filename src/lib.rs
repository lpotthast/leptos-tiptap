//! Leptos bindings for Tiptap editors.
//!
//! The crate exposes a Leptos component and hook for mounting a browser-side
//! Tiptap editor, plus handles for sending commands and reading or replacing
//! editor content.

mod api;
mod protocol;
mod runtime;

#[cfg(feature = "component")]
pub use api::component::TiptapEditor;
pub use api::{
    TiptapAttributes, TiptapCodeBlockAttributes, TiptapContent, TiptapEditorError,
    TiptapEditorHandle, TiptapEditorInstance, TiptapEditorReport, TiptapEditorResult,
    TiptapExtension, TiptapFocusOptions, TiptapFocusTarget, TiptapHeadingLevel,
    TiptapHighlightAttributes, TiptapImageResource, TiptapInsertContentOptions, TiptapLinkResource,
    TiptapListKind, TiptapMarkName, TiptapMarkOptions, TiptapNodeName, TiptapParseOptions,
    TiptapPositionOrRange, TiptapRange, TiptapSchemaTarget, TiptapSelectionState,
    TiptapSetContentOptions, TiptapSplitBlockOptions, TiptapTextAlign, TiptapToggleListOptions,
    TiptapWhitespaceMode, TiptapYoutubeVideoResource, UseTiptapEditorAttrs, UseTiptapEditorInput,
    UseTiptapEditorProps, UseTiptapEditorReturn, use_tiptap_editor,
};

/// Hook exports grouped under a module for namespaced imports.
pub mod use_tiptap_editor {
    pub use crate::{
        UseTiptapEditorAttrs, UseTiptapEditorInput, UseTiptapEditorProps, UseTiptapEditorReturn,
        use_tiptap_editor,
    };
}
