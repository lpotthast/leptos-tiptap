use serde::{Deserialize, Serialize};

/// State of the current selection.
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TiptapSelectionState {
    #[cfg(feature = "heading")]
    /// Whether the selection is in a level-1 heading.
    pub h1: bool,
    #[cfg(feature = "heading")]
    /// Whether the selection is in a level-2 heading.
    pub h2: bool,
    #[cfg(feature = "heading")]
    /// Whether the selection is in a level-3 heading.
    pub h3: bool,
    #[cfg(feature = "heading")]
    /// Whether the selection is in a level-4 heading.
    pub h4: bool,
    #[cfg(feature = "heading")]
    /// Whether the selection is in a level-5 heading.
    pub h5: bool,
    #[cfg(feature = "heading")]
    /// Whether the selection is in a level-6 heading.
    pub h6: bool,
    #[cfg(feature = "paragraph")]
    /// Whether the selection is in a paragraph.
    pub paragraph: bool,
    #[cfg(feature = "bold")]
    /// Whether bold is active in the selection.
    pub bold: bool,
    #[cfg(feature = "italic")]
    /// Whether italic is active in the selection.
    pub italic: bool,
    #[cfg(feature = "strike")]
    /// Whether strike is active in the selection.
    pub strike: bool,
    #[cfg(feature = "blockquote")]
    /// Whether the selection is in a blockquote.
    pub blockquote: bool,
    #[cfg(feature = "highlight")]
    /// Whether highlight is active in the selection.
    pub highlight: bool,
    #[cfg(feature = "bullet_list")]
    /// Whether the selection is in a bullet list.
    pub bullet_list: bool,
    #[cfg(feature = "ordered_list")]
    /// Whether the selection is in an ordered list.
    pub ordered_list: bool,
    #[cfg(feature = "text_align")]
    /// Whether left alignment is active in the selection.
    pub align_left: bool,
    #[cfg(feature = "text_align")]
    /// Whether center alignment is active in the selection.
    pub align_center: bool,
    #[cfg(feature = "text_align")]
    /// Whether right alignment is active in the selection.
    pub align_right: bool,
    #[cfg(feature = "text_align")]
    /// Whether justified alignment is active in the selection.
    pub align_justify: bool,
    #[cfg(feature = "link")]
    /// Whether link is active in the selection.
    pub link: bool,
    #[cfg(feature = "youtube")]
    /// Whether the selection is in a `YouTube` node.
    pub youtube: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;
    use serde_json::json;

    #[test]
    fn deserializes_sparse_selection_state() {
        let state: TiptapSelectionState = serde_json::from_value(json!({})).unwrap();

        assert_that!(state).is_equal_to(TiptapSelectionState::default());
    }

    #[test]
    fn ignores_unknown_selection_state_fields() {
        let state: TiptapSelectionState =
            serde_json::from_value(json!({"not_a_selection_key": true})).unwrap();

        assert_that!(state).is_equal_to(TiptapSelectionState::default());
    }

    #[cfg(feature = "bold")]
    #[test]
    fn deserializes_enabled_extension_selection_fields() {
        let state: TiptapSelectionState = serde_json::from_value(json!({"bold": true})).unwrap();

        assert_that!(state.bold).is_true();
    }

    #[cfg(not(feature = "bold"))]
    #[test]
    fn ignores_disabled_extension_selection_fields() {
        let state: TiptapSelectionState = serde_json::from_value(json!({"bold": true})).unwrap();

        assert_that!(state).is_equal_to(TiptapSelectionState::default());
    }
}
