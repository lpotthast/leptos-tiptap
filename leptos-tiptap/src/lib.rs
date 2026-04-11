mod api;
mod protocol;
mod runtime;

#[cfg(feature = "component")]
pub use api::component::TiptapEditor;
pub use api::{
    TiptapAttributes, TiptapCodeBlockAttributes, TiptapContent, TiptapEditor, TiptapEditorError,
    TiptapEditorHandle, TiptapExtension, TiptapFocusOptions, TiptapFocusTarget, TiptapHeadingLevel,
    TiptapHighlightAttributes, TiptapImageResource, TiptapInsertContentOptions, TiptapLinkResource,
    TiptapListKind, TiptapMarkName, TiptapMarkOptions, TiptapNodeName, TiptapParseOptions,
    TiptapPositionOrRange, TiptapRange, TiptapSchemaTarget, TiptapSelectionState,
    TiptapSetContentOptions, TiptapSplitBlockOptions, TiptapTextAlign, TiptapToggleListOptions,
    TiptapWhitespaceMode, TiptapYoutubeVideoResource, UseTiptapEditorAttrs, UseTiptapEditorInput,
    UseTiptapEditorProps, UseTiptapEditorReturn, use_tiptap_editor,
};

pub mod use_tiptap_editor {
    pub use crate::{
        UseTiptapEditorAttrs, UseTiptapEditorInput, UseTiptapEditorProps, UseTiptapEditorReturn,
        use_tiptap_editor,
    };
}
