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
    TiptapAttributes, TiptapContent, TiptapEditorError, TiptapEditorHandle, TiptapEditorInstance,
    TiptapFocusOptions, TiptapFocusTarget, TiptapInsertContentOptions, TiptapListKind,
    TiptapMarkName, TiptapMarkOptions, TiptapNodeName, TiptapPositionOrRange, TiptapRange,
    TiptapSchemaTarget, TiptapSplitBlockOptions, TiptapToggleListOptions,
};
use crate::protocol::{ContentPayload, EditorCommand};

macro_rules! dispatch_no_arg_methods {
    ($($(#[$meta:meta])* $method_name:ident => $command_variant:ident),* $(,)?) => {
        $(
            $(#[$meta])*
            pub fn $method_name(&self) -> Result<(), TiptapEditorError> {
                self.dispatch(EditorCommand::$command_variant)
            }
        )*
    };
}

macro_rules! delegate_no_arg_methods {
    ($($(#[$meta:meta])* $method_name:ident),* $(,)?) => {
        $(
            $(#[$meta])*
            pub fn $method_name(&self) -> Result<(), TiptapEditorError> {
                self.with_instance(TiptapEditorInstance::$method_name)
            }
        )*
    };
}

#[allow(missing_docs, clippy::missing_errors_doc)]
impl TiptapEditorInstance {
    dispatch_no_arg_methods!(
        blur => Blur,
        clear_nodes => ClearNodes,
        create_paragraph_near => CreateParagraphNear,
        delete_current_node => DeleteCurrentNode,
        delete_selection => DeleteSelection,
        enter => Enter,
        exit_code => ExitCode,
        join_up => JoinUp,
        join_down => JoinDown,
        join_backward => JoinBackward,
        join_forward => JoinForward,
        join_item_backward => JoinItemBackward,
        join_item_forward => JoinItemForward,
        join_textblock_backward => JoinTextblockBackward,
        join_textblock_forward => JoinTextblockForward,
        lift_empty_block => LiftEmptyBlock,
        newline_in_code => NewlineInCode,
        scroll_into_view => ScrollIntoView,
        select_all => SelectAll,
        select_node_backward => SelectNodeBackward,
        select_node_forward => SelectNodeForward,
        select_parent_node => SelectParentNode,
        select_textblock_end => SelectTextblockEnd,
        select_textblock_start => SelectTextblockStart,
        undo_input_rule => UndoInputRule,
        unset_all_marks => UnsetAllMarks,
        #[cfg(feature = "blockquote")]
        set_blockquote => SetBlockquote,
        #[cfg(feature = "blockquote")]
        toggle_blockquote => ToggleBlockquote,
        #[cfg(feature = "blockquote")]
        unset_blockquote => UnsetBlockquote,
        #[cfg(feature = "bold")]
        set_bold => SetBold,
        #[cfg(feature = "bold")]
        toggle_bold => ToggleBold,
        #[cfg(feature = "bold")]
        unset_bold => UnsetBold,
        #[cfg(feature = "code")]
        set_code => SetCode,
        #[cfg(feature = "code")]
        toggle_code => ToggleCode,
        #[cfg(feature = "code")]
        unset_code => UnsetCode,
        #[cfg(feature = "hard_break")]
        set_hard_break => SetHardBreak,
        #[cfg(feature = "paragraph")]
        set_paragraph => SetParagraph,
        #[cfg(feature = "highlight")]
        unset_highlight => UnsetHighlight,
        #[cfg(feature = "horizontal_rule")]
        set_horizontal_rule => SetHorizontalRule,
        #[cfg(feature = "italic")]
        set_italic => SetItalic,
        #[cfg(feature = "italic")]
        toggle_italic => ToggleItalic,
        #[cfg(feature = "italic")]
        unset_italic => UnsetItalic,
        #[cfg(feature = "list_item")]
        sink_list_item => SinkListItem,
        #[cfg(feature = "list_item")]
        lift_list_item => LiftListItem,
        #[cfg(feature = "history")]
        undo => Undo,
        #[cfg(feature = "history")]
        redo => Redo,
        #[cfg(feature = "strike")]
        set_strike => SetStrike,
        #[cfg(feature = "strike")]
        toggle_strike => ToggleStrike,
        #[cfg(feature = "strike")]
        unset_strike => UnsetStrike,
        #[cfg(feature = "bullet_list")]
        toggle_bullet_list => ToggleBulletList,
        #[cfg(feature = "ordered_list")]
        toggle_ordered_list => ToggleOrderedList,
        #[cfg(feature = "text_align")]
        unset_text_align => UnsetTextAlign,
        #[cfg(feature = "link")]
        unset_link => UnsetLink,
    );

    pub fn clear_content(&self, emit_update: bool) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ClearContent {
            emit_update: Some(emit_update),
        })
    }

    pub fn cut(&self, range: TiptapRange, target_pos: u32) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::Cut { range, target_pos })
    }

    pub fn delete_node(&self, node: TiptapNodeName) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::DeleteNode {
            type_or_name: node.schema_name().to_owned(),
        })
    }

    pub fn delete_range(&self, range: TiptapRange) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::DeleteRange { range })
    }

    pub fn extend_mark_range(
        &self,
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ExtendMarkRange {
            type_or_name: mark.schema_name().to_owned(),
            attributes,
        })
    }

    pub fn focus(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::Focus {
            target: None,
            options: None,
        })
    }

    pub fn focus_with(
        &self,
        target: TiptapFocusTarget,
        options: Option<TiptapFocusOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::Focus {
            target: Some(target.into()),
            options: options.map(Into::into),
        })
    }

    pub fn insert_content(
        &self,
        content: TiptapContent,
        options: Option<TiptapInsertContentOptions>,
    ) -> Result<(), TiptapEditorError> {
        let content = ContentPayload::try_from(content)?;
        self.dispatch(EditorCommand::InsertContent {
            content,
            options: options.map(Into::into),
        })
    }

    pub fn insert_content_at(
        &self,
        position: impl Into<TiptapPositionOrRange>,
        content: TiptapContent,
        options: Option<TiptapInsertContentOptions>,
    ) -> Result<(), TiptapEditorError> {
        let content = ContentPayload::try_from(content)?;
        self.dispatch(EditorCommand::InsertContentAt {
            position: position.into().into(),
            content,
            options: options.map(Into::into),
        })
    }

    pub fn keyboard_shortcut(&self, name: impl Into<String>) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::KeyboardShortcut { name: name.into() })
    }

    pub fn lift(
        &self,
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::Lift {
            type_or_name: node.schema_name().to_owned(),
            attributes,
        })
    }

    pub fn reset_attributes<I, S>(
        &self,
        target: TiptapSchemaTarget,
        attribute_names: I,
    ) -> Result<(), TiptapEditorError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.dispatch(EditorCommand::ResetAttributes {
            type_or_name: target.schema_name().to_owned(),
            attribute_names: attribute_names.into_iter().map(Into::into).collect(),
        })
    }

    pub fn set_mark(
        &self,
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetMark {
            type_or_name: mark.schema_name().to_owned(),
            attributes,
        })
    }

    pub fn set_meta(
        &self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetMeta {
            key: key.into(),
            value: value.into(),
        })
    }

    pub fn set_node(
        &self,
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetNode {
            type_or_name: node.schema_name().to_owned(),
            attributes,
        })
    }

    pub fn set_node_selection(&self, position: u32) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetNodeSelection { position })
    }

    pub fn set_text_selection(
        &self,
        position: impl Into<TiptapPositionOrRange>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetTextSelection {
            position: position.into().into(),
        })
    }

    pub fn split_block(
        &self,
        options: Option<TiptapSplitBlockOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SplitBlock {
            keep_marks: options.and_then(|options| options.keep_marks),
        })
    }

    pub fn toggle_list(
        &self,
        list: TiptapListKind,
        options: Option<TiptapToggleListOptions>,
    ) -> Result<(), TiptapEditorError> {
        let (keep_marks, attributes) = match options {
            Some(options) => (options.keep_marks, options.attributes),
            None => (None, None),
        };

        self.dispatch(EditorCommand::ToggleList {
            list_type_or_name: list.list_name().to_owned(),
            item_type_or_name: TiptapListKind::item_name().to_owned(),
            keep_marks,
            attributes,
        })
    }

    pub fn toggle_mark(
        &self,
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
        options: Option<TiptapMarkOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ToggleMark {
            type_or_name: mark.schema_name().to_owned(),
            attributes,
            options: options.map(Into::into),
        })
    }

    pub fn toggle_node(
        &self,
        node: TiptapNodeName,
        toggle_node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ToggleNode {
            type_or_name: node.schema_name().to_owned(),
            toggle_type_or_name: toggle_node.schema_name().to_owned(),
            attributes,
        })
    }

    pub fn toggle_wrap(
        &self,
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ToggleWrap {
            type_or_name: node.schema_name().to_owned(),
            attributes,
        })
    }

    pub fn unset_mark(
        &self,
        mark: TiptapMarkName,
        options: Option<TiptapMarkOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::UnsetMark {
            type_or_name: mark.schema_name().to_owned(),
            options: options.map(Into::into),
        })
    }

    pub fn update_attributes(
        &self,
        target: TiptapSchemaTarget,
        attributes: TiptapAttributes,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::UpdateAttributes {
            type_or_name: target.schema_name().to_owned(),
            attributes,
        })
    }

    pub fn wrap_in(
        &self,
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::WrapIn {
            type_or_name: node.schema_name().to_owned(),
            attributes,
        })
    }

    pub fn wrap_in_list(
        &self,
        list: TiptapListKind,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::WrapInList {
            type_or_name: list.list_name().to_owned(),
            attributes,
        })
    }

    #[cfg(feature = "code_block")]
    pub fn set_code_block(
        &self,
        attributes: Option<TiptapCodeBlockAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetCodeBlock { attributes })
    }

    #[cfg(feature = "code_block")]
    pub fn toggle_code_block(
        &self,
        attributes: Option<TiptapCodeBlockAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ToggleCodeBlock { attributes })
    }

    #[cfg(feature = "heading")]
    pub fn set_heading(&self, level: TiptapHeadingLevel) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetHeading {
            level: level.into(),
        })
    }

    #[cfg(feature = "heading")]
    pub fn toggle_heading(&self, level: TiptapHeadingLevel) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ToggleHeading {
            level: level.into(),
        })
    }

    #[cfg(feature = "highlight")]
    pub fn set_highlight(
        &self,
        attributes: Option<TiptapHighlightAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetHighlight { attributes })
    }

    #[cfg(feature = "highlight")]
    pub fn toggle_highlight(
        &self,
        attributes: Option<TiptapHighlightAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ToggleHighlight { attributes })
    }

    #[cfg(feature = "list_item")]
    pub fn split_list_item(
        &self,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SplitListItem { attributes })
    }

    #[cfg(feature = "text_align")]
    pub fn set_text_align(&self, alignment: TiptapTextAlign) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetTextAlign { alignment })
    }

    #[cfg(feature = "text_align")]
    pub fn toggle_text_align(&self, alignment: TiptapTextAlign) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ToggleTextAlign { alignment })
    }

    #[cfg(feature = "image")]
    pub fn set_image(&self, image: TiptapImageResource) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetImage {
            src: image.src,
            alt: image.alt,
            title: image.title,
        })
    }

    #[cfg(feature = "link")]
    pub fn set_link(&self, link: TiptapLinkResource) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetLink {
            href: link.href,
            target: link.target,
            rel: link.rel,
            class: link.class,
        })
    }

    #[cfg(feature = "link")]
    pub fn toggle_link(&self, link: TiptapLinkResource) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::ToggleLink {
            href: link.href,
            target: link.target,
            rel: link.rel,
            class: link.class,
        })
    }

    #[cfg(feature = "youtube")]
    pub fn set_youtube_video(
        &self,
        video: TiptapYoutubeVideoResource,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(EditorCommand::SetYoutubeVideo {
            src: video.src,
            start: video.start,
            width: video.width,
            height: video.height,
        })
    }
}

#[allow(missing_docs, clippy::missing_errors_doc)]
impl TiptapEditorHandle {
    delegate_no_arg_methods!(
        blur,
        clear_nodes,
        create_paragraph_near,
        delete_current_node,
        delete_selection,
        enter,
        exit_code,
        join_up,
        join_down,
        join_backward,
        join_forward,
        join_item_backward,
        join_item_forward,
        join_textblock_backward,
        join_textblock_forward,
        lift_empty_block,
        newline_in_code,
        scroll_into_view,
        select_all,
        select_node_backward,
        select_node_forward,
        select_parent_node,
        select_textblock_end,
        select_textblock_start,
        undo_input_rule,
        unset_all_marks,
        focus,
        #[cfg(feature = "blockquote")]
        set_blockquote,
        #[cfg(feature = "blockquote")]
        toggle_blockquote,
        #[cfg(feature = "blockquote")]
        unset_blockquote,
        #[cfg(feature = "bold")]
        set_bold,
        #[cfg(feature = "bold")]
        toggle_bold,
        #[cfg(feature = "bold")]
        unset_bold,
        #[cfg(feature = "code")]
        set_code,
        #[cfg(feature = "code")]
        toggle_code,
        #[cfg(feature = "code")]
        unset_code,
        #[cfg(feature = "hard_break")]
        set_hard_break,
        #[cfg(feature = "paragraph")]
        set_paragraph,
        #[cfg(feature = "highlight")]
        unset_highlight,
        #[cfg(feature = "horizontal_rule")]
        set_horizontal_rule,
        #[cfg(feature = "italic")]
        set_italic,
        #[cfg(feature = "italic")]
        toggle_italic,
        #[cfg(feature = "italic")]
        unset_italic,
        #[cfg(feature = "list_item")]
        sink_list_item,
        #[cfg(feature = "list_item")]
        lift_list_item,
        #[cfg(feature = "history")]
        undo,
        #[cfg(feature = "history")]
        redo,
        #[cfg(feature = "strike")]
        set_strike,
        #[cfg(feature = "strike")]
        toggle_strike,
        #[cfg(feature = "strike")]
        unset_strike,
        #[cfg(feature = "bullet_list")]
        toggle_bullet_list,
        #[cfg(feature = "ordered_list")]
        toggle_ordered_list,
        #[cfg(feature = "text_align")]
        unset_text_align,
        #[cfg(feature = "link")]
        unset_link,
    );

    pub fn clear_content(&self, emit_update: bool) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.clear_content(emit_update))
    }

    pub fn cut(&self, range: TiptapRange, target_pos: u32) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.cut(range, target_pos))
    }

    pub fn delete_node(&self, node: TiptapNodeName) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.delete_node(node))
    }

    pub fn delete_range(&self, range: TiptapRange) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.delete_range(range))
    }

    pub fn extend_mark_range(
        &self,
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.extend_mark_range(mark, attributes))
    }

    pub fn focus_with(
        &self,
        target: TiptapFocusTarget,
        options: Option<TiptapFocusOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.focus_with(target, options))
    }

    pub fn insert_content(
        &self,
        content: TiptapContent,
        options: Option<TiptapInsertContentOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.insert_content(content, options))
    }

    pub fn insert_content_at(
        &self,
        position: impl Into<TiptapPositionOrRange>,
        content: TiptapContent,
        options: Option<TiptapInsertContentOptions>,
    ) -> Result<(), TiptapEditorError> {
        let position = position.into();
        self.with_instance(|instance| instance.insert_content_at(position, content, options))
    }

    pub fn keyboard_shortcut(&self, name: impl Into<String>) -> Result<(), TiptapEditorError> {
        let name = name.into();
        self.with_instance(|instance| instance.keyboard_shortcut(name.clone()))
    }

    pub fn lift(
        &self,
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.lift(node, attributes))
    }

    pub fn reset_attributes<I, S>(
        &self,
        target: TiptapSchemaTarget,
        attribute_names: I,
    ) -> Result<(), TiptapEditorError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let names: Vec<String> = attribute_names.into_iter().map(Into::into).collect();
        self.with_instance(|instance| instance.reset_attributes(target, names.clone()))
    }

    pub fn set_mark(
        &self,
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_mark(mark, attributes))
    }

    pub fn set_meta(
        &self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Result<(), TiptapEditorError> {
        let key = key.into();
        let value = value.into();
        self.with_instance(|instance| instance.set_meta(key.clone(), value.clone()))
    }

    pub fn set_node(
        &self,
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_node(node, attributes))
    }

    pub fn set_node_selection(&self, position: u32) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_node_selection(position))
    }

    pub fn set_text_selection(
        &self,
        position: impl Into<TiptapPositionOrRange>,
    ) -> Result<(), TiptapEditorError> {
        let position = position.into();
        self.with_instance(|instance| instance.set_text_selection(position))
    }

    pub fn split_block(
        &self,
        options: Option<TiptapSplitBlockOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.split_block(options))
    }

    pub fn toggle_list(
        &self,
        list: TiptapListKind,
        options: Option<TiptapToggleListOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_list(list, options))
    }

    pub fn toggle_mark(
        &self,
        mark: TiptapMarkName,
        attributes: Option<TiptapAttributes>,
        options: Option<TiptapMarkOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_mark(mark, attributes, options))
    }

    pub fn toggle_node(
        &self,
        node: TiptapNodeName,
        toggle_node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_node(node, toggle_node, attributes))
    }

    pub fn toggle_wrap(
        &self,
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_wrap(node, attributes))
    }

    pub fn unset_mark(
        &self,
        mark: TiptapMarkName,
        options: Option<TiptapMarkOptions>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.unset_mark(mark, options))
    }

    pub fn update_attributes(
        &self,
        target: TiptapSchemaTarget,
        attributes: TiptapAttributes,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.update_attributes(target, attributes))
    }

    pub fn wrap_in(
        &self,
        node: TiptapNodeName,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.wrap_in(node, attributes))
    }

    pub fn wrap_in_list(
        &self,
        list: TiptapListKind,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.wrap_in_list(list, attributes))
    }

    #[cfg(feature = "code_block")]
    pub fn set_code_block(
        &self,
        attributes: Option<TiptapCodeBlockAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_code_block(attributes))
    }

    #[cfg(feature = "code_block")]
    pub fn toggle_code_block(
        &self,
        attributes: Option<TiptapCodeBlockAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_code_block(attributes))
    }

    #[cfg(feature = "heading")]
    pub fn set_heading(&self, level: TiptapHeadingLevel) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_heading(level))
    }

    #[cfg(feature = "heading")]
    pub fn toggle_heading(&self, level: TiptapHeadingLevel) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_heading(level))
    }

    #[cfg(feature = "highlight")]
    pub fn set_highlight(
        &self,
        attributes: Option<TiptapHighlightAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_highlight(attributes))
    }

    #[cfg(feature = "highlight")]
    pub fn toggle_highlight(
        &self,
        attributes: Option<TiptapHighlightAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_highlight(attributes))
    }

    #[cfg(feature = "list_item")]
    pub fn split_list_item(
        &self,
        attributes: Option<TiptapAttributes>,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.split_list_item(attributes))
    }

    #[cfg(feature = "text_align")]
    pub fn set_text_align(&self, alignment: TiptapTextAlign) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_text_align(alignment))
    }

    #[cfg(feature = "text_align")]
    pub fn toggle_text_align(&self, alignment: TiptapTextAlign) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_text_align(alignment))
    }

    #[cfg(feature = "image")]
    pub fn set_image(&self, image: TiptapImageResource) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_image(image))
    }

    #[cfg(feature = "link")]
    pub fn set_link(&self, link: TiptapLinkResource) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_link(link))
    }

    #[cfg(feature = "link")]
    pub fn toggle_link(&self, link: TiptapLinkResource) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.toggle_link(link))
    }

    #[cfg(feature = "youtube")]
    pub fn set_youtube_video(
        &self,
        video: TiptapYoutubeVideoResource,
    ) -> Result<(), TiptapEditorError> {
        self.with_instance(|instance| instance.set_youtube_video(video))
    }
}
