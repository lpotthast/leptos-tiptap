#[cfg(feature = "code_block")]
use super::TiptapCodeBlockAttributes;
#[cfg(feature = "heading")]
use super::TiptapHeadingLevel;
#[cfg(feature = "highlight")]
use super::TiptapHighlightAttributes;
#[cfg(feature = "image")]
use super::TiptapImageResource;
#[cfg(feature = "link")]
use super::TiptapLinkResource;
#[cfg(feature = "text_align")]
use super::TiptapTextAlign;
#[cfg(feature = "youtube")]
use super::TiptapYoutubeVideoResource;
use super::{
    TiptapAttributes, TiptapContent, TiptapEditorHandle, TiptapEditorInstance, TiptapEditorResult,
    TiptapFocusOptions, TiptapFocusTarget, TiptapInsertContentOptions, TiptapListKind,
    TiptapMarkName, TiptapMarkOptions, TiptapNodeName, TiptapPositionOrRange, TiptapRange,
    TiptapSchemaTarget, TiptapSplitBlockOptions, TiptapToggleListOptions,
};
use crate::protocol::{ContentPayload, EditorCommand};

macro_rules! dispatch_command {
    ($receiver:expr, $command_variant:ident) => {
        $receiver.dispatch(EditorCommand::$command_variant)
    };
    ($receiver:expr, $command_variant:ident, |$instance:ident| $body:block) => {{
        let $instance = $receiver;
        $body
    }};
}

macro_rules! command_methods {
    (
        $(
            $(#[$meta:meta])*
            $method_name:ident $(<$($generic:ident),+>)? (
                $($argument_name:ident: $argument_type:ty),* $(,)?
            ) => $command_variant:ident
            $(where [$($where_clause:tt)*])?
            $(|$instance:ident| $body:block)?
        ),* $(,)?
    ) => {
        impl TiptapEditorInstance {
            $(
                $(#[$meta])*
                #[doc = concat!(
                    "Dispatches Tiptap's `",
                    stringify!($method_name),
                    "` command on this editor instance.",
                )]
                ///
                /// # Errors
                ///
                /// Returns a [`TiptapEditorError`](crate::TiptapEditorError) if the command
                /// cannot be prepared or dispatched, for example because its input is invalid,
                /// this instance is stale, or the bridge rejected the command.
                pub fn $method_name $(<$($generic),+>)? (
                    &self,
                    $($argument_name: $argument_type),*
                ) -> TiptapEditorResult<()>
                $(where $($where_clause)*)?
                {
                    dispatch_command!(
                        self,
                        $command_variant
                        $(, |$instance| $body)?
                    )
                }
            )*
        }

        impl TiptapEditorHandle {
            $(
                $(#[$meta])*
                #[doc = concat!(
                    "Forwards to [`TiptapEditorInstance::",
                    stringify!($method_name),
                    "`] when this handle has a live editor instance.",
                )]
                ///
                /// # Errors
                ///
                /// Returns [`NotReady`](crate::TiptapEditorError::NotReady),
                /// [`Destroyed`](crate::TiptapEditorError::Destroyed), or
                /// [`CreateFailed`](crate::TiptapEditorError::CreateFailed) when the handle has no
                /// live instance; otherwise propagates errors from the instance method.
                pub fn $method_name $(<$($generic),+>)? (
                    &self,
                    $($argument_name: $argument_type),*
                ) -> TiptapEditorResult<()>
                $(where $($where_clause)*)?
                {
                    self.with_instance(move |instance| {
                        instance.$method_name($($argument_name),*)
                    })
                }
            )*
        }
    };
}

command_methods!(
    blur() => Blur,
    clear_nodes() => ClearNodes,
    create_paragraph_near() => CreateParagraphNear,
    delete_current_node() => DeleteCurrentNode,
    delete_selection() => DeleteSelection,
    enter() => Enter,
    exit_code() => ExitCode,
    join_up() => JoinUp,
    join_down() => JoinDown,
    join_backward() => JoinBackward,
    join_forward() => JoinForward,
    join_item_backward() => JoinItemBackward,
    join_item_forward() => JoinItemForward,
    join_textblock_backward() => JoinTextblockBackward,
    join_textblock_forward() => JoinTextblockForward,
    lift_empty_block() => LiftEmptyBlock,
    newline_in_code() => NewlineInCode,
    scroll_into_view() => ScrollIntoView,
    select_all() => SelectAll,
    select_node_backward() => SelectNodeBackward,
    select_node_forward() => SelectNodeForward,
    select_parent_node() => SelectParentNode,
    select_textblock_end() => SelectTextblockEnd,
    select_textblock_start() => SelectTextblockStart,
    undo_input_rule() => UndoInputRule,
    unset_all_marks() => UnsetAllMarks,
    #[cfg(feature = "blockquote")]
    set_blockquote() => SetBlockquote,
    #[cfg(feature = "blockquote")]
    toggle_blockquote() => ToggleBlockquote,
    #[cfg(feature = "blockquote")]
    unset_blockquote() => UnsetBlockquote,
    #[cfg(feature = "bold")]
    set_bold() => SetBold,
    #[cfg(feature = "bold")]
    toggle_bold() => ToggleBold,
    #[cfg(feature = "bold")]
    unset_bold() => UnsetBold,
    #[cfg(feature = "code")]
    set_code() => SetCode,
    #[cfg(feature = "code")]
    toggle_code() => ToggleCode,
    #[cfg(feature = "code")]
    unset_code() => UnsetCode,
    #[cfg(feature = "hard_break")]
    set_hard_break() => SetHardBreak,
    #[cfg(feature = "paragraph")]
    set_paragraph() => SetParagraph,
    #[cfg(feature = "highlight")]
    unset_highlight() => UnsetHighlight,
    #[cfg(feature = "horizontal_rule")]
    set_horizontal_rule() => SetHorizontalRule,
    #[cfg(feature = "italic")]
    set_italic() => SetItalic,
    #[cfg(feature = "italic")]
    toggle_italic() => ToggleItalic,
    #[cfg(feature = "italic")]
    unset_italic() => UnsetItalic,
    #[cfg(feature = "list_item")]
    sink_list_item() => SinkListItem,
    #[cfg(feature = "list_item")]
    lift_list_item() => LiftListItem,
    #[cfg(feature = "history")]
    undo() => Undo,
    #[cfg(feature = "history")]
    redo() => Redo,
    #[cfg(feature = "strike")]
    set_strike() => SetStrike,
    #[cfg(feature = "strike")]
    toggle_strike() => ToggleStrike,
    #[cfg(feature = "strike")]
    unset_strike() => UnsetStrike,
    #[cfg(feature = "bullet_list")]
    toggle_bullet_list() => ToggleBulletList,
    #[cfg(feature = "ordered_list")]
    toggle_ordered_list() => ToggleOrderedList,
    #[cfg(feature = "text_align")]
    unset_text_align() => UnsetTextAlign,
    #[cfg(feature = "link")]
    unset_link() => UnsetLink,
    clear_content(emit_update: bool) => ClearContent |instance| {
        instance.dispatch(EditorCommand::ClearContent {
            emit_update: Some(emit_update),
        })
    },
    cut(range: TiptapRange, target_pos: u32) => Cut |instance| {
        instance.dispatch(EditorCommand::Cut { range, target_pos })
    },
    delete_node(node: TiptapNodeName) => DeleteNode |instance| {
        instance.dispatch(EditorCommand::DeleteNode {
            type_or_name: node.schema_name().to_owned(),
        })
    },
    delete_range(range: TiptapRange) => DeleteRange |instance| {
        instance.dispatch(EditorCommand::DeleteRange { range })
    },
    extend_mark_range(
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
    ) => ExtendMarkRange |instance| {
        instance.dispatch(EditorCommand::ExtendMarkRange {
            type_or_name: mark.schema_name().to_owned(),
            attributes,
        })
    },
    focus() => Focus |instance| {
        instance.dispatch(EditorCommand::Focus {
            target: None,
            options: None,
        })
    },
    focus_with(
        target: TiptapFocusTarget,
        options: Option<TiptapFocusOptions>,
    ) => Focus |instance| {
        instance.dispatch(EditorCommand::Focus {
            target: Some(target.into()),
            options: options.map(Into::into),
        })
    },
    insert_content(
        content: TiptapContent,
        options: Option<TiptapInsertContentOptions>,
    ) => InsertContent |instance| {
        let content = ContentPayload::try_from(content)?;
        instance.dispatch(EditorCommand::InsertContent {
            content,
            options: options.map(Into::into),
        })
    },
    insert_content_at(
        position: impl Into<TiptapPositionOrRange>,
        content: TiptapContent,
        options: Option<TiptapInsertContentOptions>,
    ) => InsertContentAt |instance| {
        let content = ContentPayload::try_from(content)?;
        instance.dispatch(EditorCommand::InsertContentAt {
            position: position.into().into(),
            content,
            options: options.map(Into::into),
        })
    },
    keyboard_shortcut(name: impl Into<String>) => KeyboardShortcut |instance| {
        instance.dispatch(EditorCommand::KeyboardShortcut { name: name.into() })
    },
    lift(
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) => Lift |instance| {
        instance.dispatch(EditorCommand::Lift {
            type_or_name: node.schema_name().to_owned(),
            attributes,
        })
    },
    reset_attributes<I, S>(
        target: TiptapSchemaTarget,
        attribute_names: I,
    ) => ResetAttributes
    where [
        I: IntoIterator<Item = S>,
        S: Into<String>,
    ]
    |instance| {
        instance.dispatch(EditorCommand::ResetAttributes {
            type_or_name: target.schema_name().to_owned(),
            attribute_names: attribute_names.into_iter().map(Into::into).collect(),
        })
    },
    set_mark(
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
    ) => SetMark |instance| {
        instance.dispatch(EditorCommand::SetMark {
            type_or_name: mark.schema_name().to_owned(),
            attributes,
        })
    },
    set_meta(
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) => SetMeta |instance| {
        instance.dispatch(EditorCommand::SetMeta {
            key: key.into(),
            value: value.into(),
        })
    },
    set_node(
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) => SetNode |instance| {
        instance.dispatch(EditorCommand::SetNode {
            type_or_name: node.schema_name().to_owned(),
            attributes,
        })
    },
    set_node_selection(position: u32) => SetNodeSelection |instance| {
        instance.dispatch(EditorCommand::SetNodeSelection { position })
    },
    set_text_selection(
        position: impl Into<TiptapPositionOrRange>,
    ) => SetTextSelection |instance| {
        instance.dispatch(EditorCommand::SetTextSelection {
            position: position.into().into(),
        })
    },
    split_block(
        options: Option<TiptapSplitBlockOptions>,
    ) => SplitBlock |instance| {
        instance.dispatch(EditorCommand::SplitBlock {
            keep_marks: options.and_then(|options| options.keep_marks),
        })
    },
    toggle_list(
        list: TiptapListKind,
        options: Option<TiptapToggleListOptions>,
    ) => ToggleList |instance| {
        let (keep_marks, attributes) = match options {
            Some(options) => (options.keep_marks, options.attributes),
            None => (None, None),
        };

        instance.dispatch(EditorCommand::ToggleList {
            list_type_or_name: list.list_name().to_owned(),
            item_type_or_name: TiptapListKind::item_name().to_owned(),
            keep_marks,
            attributes,
        })
    },
    toggle_mark(
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
        options: Option<TiptapMarkOptions>,
    ) => ToggleMark |instance| {
        instance.dispatch(EditorCommand::ToggleMark {
            type_or_name: mark.schema_name().to_owned(),
            attributes,
            options: options.map(Into::into),
        })
    },
    toggle_node(
        node: TiptapNodeName,
        toggle_node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) => ToggleNode |instance| {
        instance.dispatch(EditorCommand::ToggleNode {
            type_or_name: node.schema_name().to_owned(),
            toggle_type_or_name: toggle_node.schema_name().to_owned(),
            attributes,
        })
    },
    toggle_wrap(
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) => ToggleWrap |instance| {
        instance.dispatch(EditorCommand::ToggleWrap {
            type_or_name: node.schema_name().to_owned(),
            attributes,
        })
    },
    unset_mark(
        mark: TiptapMarkName,
        options: Option<TiptapMarkOptions>,
    ) => UnsetMark |instance| {
        instance.dispatch(EditorCommand::UnsetMark {
            type_or_name: mark.schema_name().to_owned(),
            options: options.map(Into::into),
        })
    },
    update_attributes(
        target: TiptapSchemaTarget,
        attributes: TiptapAttributes,
    ) => UpdateAttributes |instance| {
        instance.dispatch(EditorCommand::UpdateAttributes {
            type_or_name: target.schema_name().to_owned(),
            attributes,
        })
    },
    wrap_in(
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) => WrapIn |instance| {
        instance.dispatch(EditorCommand::WrapIn {
            type_or_name: node.schema_name().to_owned(),
            attributes,
        })
    },
    wrap_in_list(
        list: TiptapListKind,
        attributes: Option<TiptapAttributes>,
    ) => WrapInList |instance| {
        instance.dispatch(EditorCommand::WrapInList {
            type_or_name: list.list_name().to_owned(),
            attributes,
        })
    },
    #[cfg(feature = "code_block")]
    set_code_block(
        attributes: Option<TiptapCodeBlockAttributes>,
    ) => SetCodeBlock |instance| {
        instance.dispatch(EditorCommand::SetCodeBlock { attributes })
    },
    #[cfg(feature = "code_block")]
    toggle_code_block(
        attributes: Option<TiptapCodeBlockAttributes>,
    ) => ToggleCodeBlock |instance| {
        instance.dispatch(EditorCommand::ToggleCodeBlock { attributes })
    },
    #[cfg(feature = "heading")]
    set_heading(level: TiptapHeadingLevel) => SetHeading |instance| {
        instance.dispatch(EditorCommand::SetHeading {
            level: level.into(),
        })
    },
    #[cfg(feature = "heading")]
    toggle_heading(level: TiptapHeadingLevel) => ToggleHeading |instance| {
        instance.dispatch(EditorCommand::ToggleHeading {
            level: level.into(),
        })
    },
    #[cfg(feature = "highlight")]
    set_highlight(
        attributes: Option<TiptapHighlightAttributes>,
    ) => SetHighlight |instance| {
        instance.dispatch(EditorCommand::SetHighlight { attributes })
    },
    #[cfg(feature = "highlight")]
    toggle_highlight(
        attributes: Option<TiptapHighlightAttributes>,
    ) => ToggleHighlight |instance| {
        instance.dispatch(EditorCommand::ToggleHighlight { attributes })
    },
    #[cfg(feature = "list_item")]
    split_list_item(
        attributes: Option<TiptapAttributes>,
    ) => SplitListItem |instance| {
        instance.dispatch(EditorCommand::SplitListItem { attributes })
    },
    #[cfg(feature = "text_align")]
    set_text_align(alignment: TiptapTextAlign) => SetTextAlign |instance| {
        instance.dispatch(EditorCommand::SetTextAlign { alignment })
    },
    #[cfg(feature = "text_align")]
    toggle_text_align(alignment: TiptapTextAlign) => ToggleTextAlign |instance| {
        instance.dispatch(EditorCommand::ToggleTextAlign { alignment })
    },
    #[cfg(feature = "image")]
    set_image(image: TiptapImageResource) => SetImage |instance| {
        instance.dispatch(EditorCommand::SetImage {
            src: image.src,
            alt: image.alt,
            title: image.title,
        })
    },
    #[cfg(feature = "link")]
    set_link(link: TiptapLinkResource) => SetLink |instance| {
        instance.dispatch(EditorCommand::SetLink {
            href: link.href,
            target: link.target,
            rel: link.rel,
            class: link.class,
        })
    },
    #[cfg(feature = "link")]
    toggle_link(link: TiptapLinkResource) => ToggleLink |instance| {
        instance.dispatch(EditorCommand::ToggleLink {
            href: link.href,
            target: link.target,
            rel: link.rel,
            class: link.class,
        })
    },
    #[cfg(feature = "youtube")]
    set_youtube_video(
        video: TiptapYoutubeVideoResource,
    ) => SetYoutubeVideo |instance| {
        instance.dispatch(EditorCommand::SetYoutubeVideo {
            src: video.src,
            start: video.start,
            width: video.width,
            height: video.height,
        })
    },
);
