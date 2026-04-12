use serde::{Deserialize, Serialize};

#[cfg(feature = "code_block")]
use crate::TiptapCodeBlockAttributes;
#[cfg(feature = "highlight")]
use crate::TiptapHighlightAttributes;
#[cfg(feature = "text_align")]
use crate::TiptapTextAlign;
use crate::{
    TiptapAttributes, TiptapContent, TiptapEditorError, TiptapFocusOptions, TiptapFocusTarget,
    TiptapInsertContentOptions, TiptapMarkOptions, TiptapParseOptions, TiptapPositionOrRange,
    TiptapRange, TiptapSetContentOptions, TiptapWhitespaceMode,
};

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(feature = "ssr"), serde(rename_all = "snake_case"))]
pub(crate) enum ContentFormat {
    Html,
    Json,
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    not(feature = "ssr"),
    serde(tag = "format", content = "value", rename_all = "snake_case")
)]
pub(crate) enum ContentPayload {
    Html(String),
    Json(serde_json::Value),
}

impl TryFrom<TiptapContent> for ContentPayload {
    type Error = TiptapEditorError;

    fn try_from(value: TiptapContent) -> Result<Self, Self::Error> {
        match value {
            TiptapContent::Html(content) => Ok(Self::Html(content)),
            TiptapContent::Json(content) => Ok(Self::Json(content)),
        }
    }
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct CreateRequest {
    pub(crate) id: String,
    pub(crate) content: ContentPayload,
    pub(crate) editable: bool,
    pub(crate) extensions: Vec<&'static str>,
    pub(crate) placeholder: Option<String>,
}

#[cfg(not(feature = "ssr"))]
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub(crate) struct ReadyPayload {
    pub(crate) generation: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum TiptapPositionOrRangePayload {
    Position(u32),
    Range(TiptapRange),
}

impl From<TiptapPositionOrRange> for TiptapPositionOrRangePayload {
    fn from(value: TiptapPositionOrRange) -> Self {
        match value {
            TiptapPositionOrRange::Position(position) => Self::Position(position),
            TiptapPositionOrRange::Range(range) => Self::Range(range),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum TiptapFocusTargetPayload {
    Position(u32),
    Keyword(TiptapFocusKeyword),
    Current(Option<()>),
}

impl From<TiptapFocusTarget> for TiptapFocusTargetPayload {
    fn from(value: TiptapFocusTarget) -> Self {
        match value {
            TiptapFocusTarget::Current => Self::Current(None),
            TiptapFocusTarget::Start => Self::Keyword(TiptapFocusKeyword::Start),
            TiptapFocusTarget::End => Self::Keyword(TiptapFocusKeyword::End),
            TiptapFocusTarget::All => Self::Keyword(TiptapFocusKeyword::All),
            TiptapFocusTarget::At(position) => Self::Position(position),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TiptapFocusKeyword {
    Start,
    End,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum TiptapWhitespaceModePayload {
    Preserve(bool),
    Full(TiptapWhitespaceModeKeyword),
}

impl From<TiptapWhitespaceMode> for TiptapWhitespaceModePayload {
    fn from(value: TiptapWhitespaceMode) -> Self {
        match value {
            TiptapWhitespaceMode::Preserve => Self::Preserve(true),
            TiptapWhitespaceMode::Full => Self::Full(TiptapWhitespaceModeKeyword::Full),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TiptapWhitespaceModeKeyword {
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub(crate) struct TiptapParseOptionsPayload {
    pub preserve_whitespace: Option<TiptapWhitespaceModePayload>,
    pub from: Option<u32>,
    pub to: Option<u32>,
}

impl From<TiptapParseOptions> for TiptapParseOptionsPayload {
    fn from(value: TiptapParseOptions) -> Self {
        Self {
            preserve_whitespace: value.preserve_whitespace.map(Into::into),
            from: value.from,
            to: value.to,
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FocusOptionsPayload {
    pub(crate) scroll_into_view: Option<bool>,
}

impl From<TiptapFocusOptions> for FocusOptionsPayload {
    fn from(value: TiptapFocusOptions) -> Self {
        Self {
            scroll_into_view: value.scroll_into_view,
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct SetContentOptionsPayload {
    pub(crate) emit_update: Option<bool>,
    pub(crate) parse_options: Option<TiptapParseOptionsPayload>,
    pub(crate) error_on_invalid_content: Option<bool>,
}

impl From<TiptapSetContentOptions> for SetContentOptionsPayload {
    fn from(value: TiptapSetContentOptions) -> Self {
        Self {
            emit_update: value.emit_update,
            parse_options: value.parse_options.map(Into::into),
            error_on_invalid_content: value.error_on_invalid_content,
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct InsertContentOptionsPayload {
    pub(crate) parse_options: Option<TiptapParseOptionsPayload>,
    pub(crate) update_selection: Option<bool>,
    pub(crate) apply_input_rules: Option<bool>,
    pub(crate) apply_paste_rules: Option<bool>,
    pub(crate) error_on_invalid_content: Option<bool>,
}

impl From<TiptapInsertContentOptions> for InsertContentOptionsPayload {
    fn from(value: TiptapInsertContentOptions) -> Self {
        Self {
            parse_options: value.parse_options.map(Into::into),
            update_selection: value.update_selection,
            apply_input_rules: value.apply_input_rules,
            apply_paste_rules: value.apply_paste_rules,
            error_on_invalid_content: value.error_on_invalid_content,
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MarkOptionsPayload {
    pub(crate) extend_empty_mark_range: Option<bool>,
}

impl From<TiptapMarkOptions> for MarkOptionsPayload {
    fn from(value: TiptapMarkOptions) -> Self {
        Self {
            extend_empty_mark_range: value.extend_empty_mark_range,
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "ssr"), serde(tag = "kind", rename_all = "snake_case"))]
pub(crate) enum EditorCommand {
    Blur,
    ClearContent {
        emit_update: Option<bool>,
    },
    ClearNodes,
    CreateParagraphNear,
    Cut {
        range: TiptapRange,
        target_pos: u32,
    },
    DeleteCurrentNode,
    DeleteNode {
        type_or_name: String,
    },
    DeleteRange {
        range: TiptapRange,
    },
    DeleteSelection,
    Enter,
    ExitCode,
    ExtendMarkRange {
        type_or_name: String,
        attributes: Option<TiptapAttributes>,
    },
    Focus {
        target: Option<TiptapFocusTargetPayload>,
        options: Option<FocusOptionsPayload>,
    },
    InsertContent {
        content: ContentPayload,
        options: Option<InsertContentOptionsPayload>,
    },
    InsertContentAt {
        position: TiptapPositionOrRangePayload,
        content: ContentPayload,
        options: Option<InsertContentOptionsPayload>,
    },
    JoinUp,
    JoinDown,
    JoinBackward,
    JoinForward,
    JoinItemBackward,
    JoinItemForward,
    JoinTextblockBackward,
    JoinTextblockForward,
    KeyboardShortcut {
        name: String,
    },
    Lift {
        type_or_name: String,
        attributes: Option<TiptapAttributes>,
    },
    LiftEmptyBlock,
    NewlineInCode,
    ResetAttributes {
        type_or_name: String,
        attribute_names: Vec<String>,
    },
    ScrollIntoView,
    SelectAll,
    SelectNodeBackward,
    SelectNodeForward,
    SelectParentNode,
    SelectTextblockEnd,
    SelectTextblockStart,
    SetMark {
        type_or_name: String,
        attributes: Option<TiptapAttributes>,
    },
    SetMeta {
        key: String,
        value: serde_json::Value,
    },
    SetNode {
        type_or_name: String,
        attributes: Option<TiptapAttributes>,
    },
    SetNodeSelection {
        position: u32,
    },
    SetTextSelection {
        position: TiptapPositionOrRangePayload,
    },
    SplitBlock {
        keep_marks: Option<bool>,
    },
    ToggleList {
        list_type_or_name: String,
        item_type_or_name: String,
        keep_marks: Option<bool>,
        attributes: Option<TiptapAttributes>,
    },
    ToggleMark {
        type_or_name: String,
        attributes: Option<TiptapAttributes>,
        options: Option<MarkOptionsPayload>,
    },
    ToggleNode {
        type_or_name: String,
        toggle_type_or_name: String,
        attributes: Option<TiptapAttributes>,
    },
    ToggleWrap {
        type_or_name: String,
        attributes: Option<TiptapAttributes>,
    },
    UndoInputRule,
    UnsetAllMarks,
    UnsetMark {
        type_or_name: String,
        options: Option<MarkOptionsPayload>,
    },
    UpdateAttributes {
        type_or_name: String,
        attributes: TiptapAttributes,
    },
    WrapIn {
        type_or_name: String,
        attributes: Option<TiptapAttributes>,
    },
    WrapInList {
        type_or_name: String,
        attributes: Option<TiptapAttributes>,
    },
    #[cfg(feature = "blockquote")]
    SetBlockquote,
    #[cfg(feature = "blockquote")]
    ToggleBlockquote,
    #[cfg(feature = "blockquote")]
    UnsetBlockquote,
    #[cfg(feature = "heading")]
    ToggleHeading {
        level: i32,
    },
    #[cfg(feature = "heading")]
    SetHeading {
        level: i32,
    },
    #[cfg(feature = "paragraph")]
    SetParagraph,
    #[cfg(feature = "bold")]
    SetBold,
    #[cfg(feature = "bold")]
    ToggleBold,
    #[cfg(feature = "bold")]
    UnsetBold,
    #[cfg(feature = "code")]
    SetCode,
    #[cfg(feature = "code")]
    ToggleCode,
    #[cfg(feature = "code")]
    UnsetCode,
    #[cfg(feature = "code_block")]
    SetCodeBlock {
        attributes: Option<TiptapCodeBlockAttributes>,
    },
    #[cfg(feature = "code_block")]
    ToggleCodeBlock {
        attributes: Option<TiptapCodeBlockAttributes>,
    },
    #[cfg(feature = "hard_break")]
    SetHardBreak,
    #[cfg(feature = "horizontal_rule")]
    SetHorizontalRule,
    #[cfg(feature = "italic")]
    SetItalic,
    #[cfg(feature = "italic")]
    ToggleItalic,
    #[cfg(feature = "italic")]
    UnsetItalic,
    #[cfg(feature = "strike")]
    SetStrike,
    #[cfg(feature = "strike")]
    ToggleStrike,
    #[cfg(feature = "strike")]
    UnsetStrike,
    #[cfg(feature = "highlight")]
    SetHighlight {
        attributes: Option<TiptapHighlightAttributes>,
    },
    #[cfg(feature = "highlight")]
    ToggleHighlight {
        attributes: Option<TiptapHighlightAttributes>,
    },
    #[cfg(feature = "highlight")]
    UnsetHighlight,
    #[cfg(feature = "bullet_list")]
    ToggleBulletList,
    #[cfg(feature = "list_item")]
    SplitListItem {
        attributes: Option<TiptapAttributes>,
    },
    #[cfg(feature = "list_item")]
    SinkListItem,
    #[cfg(feature = "list_item")]
    LiftListItem,
    #[cfg(feature = "ordered_list")]
    ToggleOrderedList,
    #[cfg(feature = "text_align")]
    SetTextAlign {
        alignment: TiptapTextAlign,
    },
    #[cfg(feature = "text_align")]
    ToggleTextAlign {
        alignment: TiptapTextAlign,
    },
    #[cfg(feature = "text_align")]
    UnsetTextAlign,
    #[cfg(feature = "history")]
    Undo,
    #[cfg(feature = "history")]
    Redo,
    #[cfg(feature = "image")]
    SetImage {
        src: String,
        alt: Option<String>,
        title: Option<String>,
    },
    #[cfg(feature = "link")]
    SetLink {
        href: String,
        target: Option<String>,
        rel: Option<String>,
        class: Option<String>,
    },
    #[cfg(feature = "link")]
    ToggleLink {
        href: String,
        target: Option<String>,
        rel: Option<String>,
        class: Option<String>,
    },
    #[cfg(feature = "link")]
    UnsetLink,
    #[cfg(feature = "youtube")]
    SetYoutubeVideo {
        src: String,
        start: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
    },
    SetEditable {
        editable: bool,
    },
}

impl EditorCommand {
    #[cfg(not(feature = "ssr"))]
    pub(crate) fn operation_name(&self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::ClearContent { .. } => "clear_content",
            Self::ClearNodes => "clear_nodes",
            Self::CreateParagraphNear => "create_paragraph_near",
            Self::Cut { .. } => "cut",
            Self::DeleteCurrentNode => "delete_current_node",
            Self::DeleteNode { .. } => "delete_node",
            Self::DeleteRange { .. } => "delete_range",
            Self::DeleteSelection => "delete_selection",
            Self::Enter => "enter",
            Self::ExitCode => "exit_code",
            Self::ExtendMarkRange { .. } => "extend_mark_range",
            Self::Focus { .. } => "focus",
            Self::InsertContent { .. } => "insert_content",
            Self::InsertContentAt { .. } => "insert_content_at",
            Self::JoinUp => "join_up",
            Self::JoinDown => "join_down",
            Self::JoinBackward => "join_backward",
            Self::JoinForward => "join_forward",
            Self::JoinItemBackward => "join_item_backward",
            Self::JoinItemForward => "join_item_forward",
            Self::JoinTextblockBackward => "join_textblock_backward",
            Self::JoinTextblockForward => "join_textblock_forward",
            Self::KeyboardShortcut { .. } => "keyboard_shortcut",
            Self::Lift { .. } => "lift",
            Self::LiftEmptyBlock => "lift_empty_block",
            Self::NewlineInCode => "newline_in_code",
            Self::ResetAttributes { .. } => "reset_attributes",
            Self::ScrollIntoView => "scroll_into_view",
            Self::SelectAll => "select_all",
            Self::SelectNodeBackward => "select_node_backward",
            Self::SelectNodeForward => "select_node_forward",
            Self::SelectParentNode => "select_parent_node",
            Self::SelectTextblockEnd => "select_textblock_end",
            Self::SelectTextblockStart => "select_textblock_start",
            Self::SetMark { .. } => "set_mark",
            Self::SetMeta { .. } => "set_meta",
            Self::SetNode { .. } => "set_node",
            Self::SetNodeSelection { .. } => "set_node_selection",
            Self::SetTextSelection { .. } => "set_text_selection",
            Self::SplitBlock { .. } => "split_block",
            Self::ToggleList { .. } => "toggle_list",
            Self::ToggleMark { .. } => "toggle_mark",
            Self::ToggleNode { .. } => "toggle_node",
            Self::ToggleWrap { .. } => "toggle_wrap",
            Self::UndoInputRule => "undo_input_rule",
            Self::UnsetAllMarks => "unset_all_marks",
            Self::UnsetMark { .. } => "unset_mark",
            Self::UpdateAttributes { .. } => "update_attributes",
            Self::WrapIn { .. } => "wrap_in",
            Self::WrapInList { .. } => "wrap_in_list",
            #[cfg(feature = "blockquote")]
            Self::SetBlockquote => "set_blockquote",
            #[cfg(feature = "blockquote")]
            Self::ToggleBlockquote => "toggle_blockquote",
            #[cfg(feature = "blockquote")]
            Self::UnsetBlockquote => "unset_blockquote",
            #[cfg(feature = "bold")]
            Self::SetBold => "set_bold",
            #[cfg(feature = "heading")]
            Self::ToggleHeading { .. } => "toggle_heading",
            #[cfg(feature = "heading")]
            Self::SetHeading { .. } => "set_heading",
            #[cfg(feature = "paragraph")]
            Self::SetParagraph => "set_paragraph",
            #[cfg(feature = "bold")]
            Self::ToggleBold => "toggle_bold",
            #[cfg(feature = "bold")]
            Self::UnsetBold => "unset_bold",
            #[cfg(feature = "code")]
            Self::SetCode => "set_code",
            #[cfg(feature = "code")]
            Self::ToggleCode => "toggle_code",
            #[cfg(feature = "code")]
            Self::UnsetCode => "unset_code",
            #[cfg(feature = "code_block")]
            Self::SetCodeBlock { .. } => "set_code_block",
            #[cfg(feature = "code_block")]
            Self::ToggleCodeBlock { .. } => "toggle_code_block",
            #[cfg(feature = "hard_break")]
            Self::SetHardBreak => "set_hard_break",
            #[cfg(feature = "horizontal_rule")]
            Self::SetHorizontalRule => "set_horizontal_rule",
            #[cfg(feature = "italic")]
            Self::SetItalic => "set_italic",
            #[cfg(feature = "italic")]
            Self::ToggleItalic => "toggle_italic",
            #[cfg(feature = "italic")]
            Self::UnsetItalic => "unset_italic",
            #[cfg(feature = "strike")]
            Self::SetStrike => "set_strike",
            #[cfg(feature = "strike")]
            Self::ToggleStrike => "toggle_strike",
            #[cfg(feature = "strike")]
            Self::UnsetStrike => "unset_strike",
            #[cfg(feature = "highlight")]
            Self::SetHighlight { .. } => "set_highlight",
            #[cfg(feature = "highlight")]
            Self::ToggleHighlight { .. } => "toggle_highlight",
            #[cfg(feature = "highlight")]
            Self::UnsetHighlight => "unset_highlight",
            #[cfg(feature = "bullet_list")]
            Self::ToggleBulletList => "toggle_bullet_list",
            #[cfg(feature = "list_item")]
            Self::SplitListItem { .. } => "split_list_item",
            #[cfg(feature = "list_item")]
            Self::SinkListItem => "sink_list_item",
            #[cfg(feature = "list_item")]
            Self::LiftListItem => "lift_list_item",
            #[cfg(feature = "ordered_list")]
            Self::ToggleOrderedList => "toggle_ordered_list",
            #[cfg(feature = "text_align")]
            Self::SetTextAlign { .. } => "set_text_align",
            #[cfg(feature = "text_align")]
            Self::ToggleTextAlign { .. } => "toggle_text_align",
            #[cfg(feature = "text_align")]
            Self::UnsetTextAlign => "unset_text_align",
            #[cfg(feature = "history")]
            Self::Undo => "undo",
            #[cfg(feature = "history")]
            Self::Redo => "redo",
            #[cfg(feature = "image")]
            Self::SetImage { .. } => "set_image",
            #[cfg(feature = "link")]
            Self::SetLink { .. } => "set_link",
            #[cfg(feature = "link")]
            Self::ToggleLink { .. } => "toggle_link",
            #[cfg(feature = "link")]
            Self::UnsetLink => "unset_link",
            #[cfg(feature = "youtube")]
            Self::SetYoutubeVideo { .. } => "set_youtube_video",
            Self::SetEditable { .. } => "set_editable",
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "ssr"), serde(tag = "kind", rename_all = "snake_case"))]
pub(crate) enum DocumentRequest {
    GetContent {
        format: ContentFormat,
    },
    SetContent {
        content: ContentPayload,
        options: Option<SetContentOptionsPayload>,
    },
}

impl DocumentRequest {
    #[cfg(not(feature = "ssr"))]
    pub(crate) fn operation_name(&self) -> &'static str {
        match self {
            Self::GetContent {
                format: ContentFormat::Html,
            } => "get_content_html",
            Self::GetContent {
                format: ContentFormat::Json,
            } => "get_content_json",
            Self::SetContent { .. } => "set_content",
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
#[cfg_attr(not(feature = "ssr"), serde(tag = "kind", rename_all = "snake_case"))]
pub(crate) enum DocumentResponse {
    Content { content: ContentPayload },
    Empty,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum EmptyResponse {
    Empty,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Serialize)]
pub(crate) struct CommandInvocation {
    pub(crate) id: String,
    pub(crate) generation: u32,
    pub(crate) command: EditorCommand,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Serialize)]
pub(crate) struct DocumentInvocation {
    pub(crate) id: String,
    pub(crate) generation: u32,
    pub(crate) request: DocumentRequest,
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;
    use serde_json::json;

    #[test]
    fn serializes_focus_targets_to_tiptap_wire_values() {
        assert_that!(
            serde_json::to_value(TiptapFocusTargetPayload::from(TiptapFocusTarget::Current,))
                .unwrap()
        )
        .is_equal_to(json!(null));
        assert_that!(
            serde_json::to_value(TiptapFocusTargetPayload::from(TiptapFocusTarget::Start,))
                .unwrap()
        )
        .is_equal_to(json!("start"));
        assert_that!(
            serde_json::to_value(TiptapFocusTargetPayload::from(TiptapFocusTarget::At(7),))
                .unwrap()
        )
        .is_equal_to(json!(7));
    }

    #[test]
    fn serializes_parse_options_with_supported_whitespace_modes() {
        assert_that!(
            serde_json::to_value(TiptapParseOptionsPayload::from(TiptapParseOptions {
                preserve_whitespace: Some(TiptapWhitespaceMode::Preserve),
                from: Some(1),
                to: Some(2),
            }))
            .unwrap()
        )
        .is_equal_to(json!({
            "preserve_whitespace": true,
            "from": 1,
            "to": 2
        }));

        assert_that!(
            serde_json::to_value(TiptapParseOptionsPayload::from(TiptapParseOptions {
                preserve_whitespace: Some(TiptapWhitespaceMode::Full),
                from: None,
                to: None,
            }))
            .unwrap()
        )
        .is_equal_to(json!({
            "preserve_whitespace": "full",
            "from": null,
            "to": null
        }));
    }

    #[cfg(not(feature = "ssr"))]
    #[test]
    fn reports_operation_names_for_new_command_variants() {
        assert_that!(EditorCommand::Blur.operation_name()).is_equal_to("blur");
        assert_that!(
            EditorCommand::Focus {
                target: None,
                options: None,
            }
            .operation_name()
        )
        .is_equal_to("focus");
        assert_that!(
            EditorCommand::ToggleMark {
                type_or_name: "bold".to_owned(),
                attributes: None,
                options: Some(MarkOptionsPayload {
                    extend_empty_mark_range: Some(true),
                }),
            }
            .operation_name()
        )
        .is_equal_to("toggle_mark");
        #[cfg(feature = "bold")]
        assert_that!(EditorCommand::SetBold.operation_name()).is_equal_to("set_bold");
        #[cfg(feature = "bold")]
        assert_that!(EditorCommand::UnsetBold.operation_name()).is_equal_to("unset_bold");
        #[cfg(feature = "code")]
        assert_that!(EditorCommand::SetCode.operation_name()).is_equal_to("set_code");
        #[cfg(feature = "code_block")]
        assert_that!(
            EditorCommand::SetCodeBlock {
                attributes: Some(TiptapCodeBlockAttributes {
                    language: Some("rust".to_owned()),
                }),
            }
            .operation_name()
        )
        .is_equal_to("set_code_block");
        #[cfg(feature = "text_align")]
        assert_that!(
            EditorCommand::SetTextAlign {
                alignment: TiptapTextAlign::Left,
            }
            .operation_name()
        )
        .is_equal_to("set_text_align");
        #[cfg(feature = "list_item")]
        assert_that!(
            EditorCommand::SplitListItem {
                attributes: Some(
                    serde_json::from_value(json!({"checked": true}))
                        .expect("object attributes should deserialize"),
                ),
            }
            .operation_name()
        )
        .is_equal_to("split_list_item");
        #[cfg(feature = "history")]
        assert_that!(EditorCommand::Undo.operation_name()).is_equal_to("undo");
        #[cfg(feature = "history")]
        assert_that!(EditorCommand::Redo.operation_name()).is_equal_to("redo");
    }
}
