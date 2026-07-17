use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A boolean editor state that can be active at the current selection.
///
/// Keys describe the selection-state wire protocol and are available independently of Cargo
/// features. A key is only present in [`TiptapActiveState`] when an extension selected for the
/// editor contributes it.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TiptapActiveKey {
    /// A level-1 heading.
    H1,
    /// A level-2 heading.
    H2,
    /// A level-3 heading.
    H3,
    /// A level-4 heading.
    H4,
    /// A level-5 heading.
    H5,
    /// A level-6 heading.
    H6,
    /// A paragraph.
    Paragraph,
    /// The bold mark.
    Bold,
    /// The italic mark.
    Italic,
    /// The strike mark.
    Strike,
    /// A blockquote.
    Blockquote,
    /// The highlight mark.
    Highlight,
    /// A bullet list.
    BulletList,
    /// An ordered list.
    OrderedList,
    /// Left-aligned text.
    AlignLeft,
    /// Center-aligned text.
    AlignCenter,
    /// Right-aligned text.
    AlignRight,
    /// Justified text.
    AlignJustify,
    /// The link mark.
    Link,
    /// A `YouTube` node.
    Youtube,
}

impl TiptapActiveKey {
    /// Returns the stable wire name for this key.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Paragraph => "paragraph",
            Self::Bold => "bold",
            Self::Italic => "italic",
            Self::Strike => "strike",
            Self::Blockquote => "blockquote",
            Self::Highlight => "highlight",
            Self::BulletList => "bullet_list",
            Self::OrderedList => "ordered_list",
            Self::AlignLeft => "align_left",
            Self::AlignCenter => "align_center",
            Self::AlignRight => "align_right",
            Self::AlignJustify => "align_justify",
            Self::Link => "link",
            Self::Youtube => "youtube",
        }
    }
}

/// Boolean active states contributed by extensions selected for an editor.
///
/// A missing key means that the editor has no contributor for it. A present `false` value means
/// that a selected extension reports the state as inactive.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TiptapActiveState(BTreeMap<TiptapActiveKey, bool>);

impl TiptapActiveState {
    /// Returns the reported value for `key`, or `None` when no selected extension contributes it.
    #[must_use]
    pub fn get(&self, key: TiptapActiveKey) -> Option<bool> {
        self.0.get(&key).copied()
    }

    /// Returns whether `key` is reported as active.
    ///
    /// Missing keys are treated as inactive. Use [`Self::get`] when the distinction between
    /// missing and explicitly inactive matters.
    #[must_use]
    pub fn is_active(&self, key: TiptapActiveKey) -> bool {
        self.get(key).unwrap_or(false)
    }

    /// Returns all contributed active-state entries in stable key order.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (TiptapActiveKey, bool)> + DoubleEndedIterator + '_ {
        self.0.iter().map(|(&key, &active)| (key, active))
    }

    /// Returns the number of contributed active-state entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether no extension contributes an active-state entry.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// State of the current editor selection.
///
/// The aggregate is opaque so separately typed selection information can be added without
/// changing the representation exposed to applications. Boolean extension activity is available
/// through [`Self::active_state`], [`Self::active`], and [`Self::is_active`].
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TiptapSelectionState {
    active: TiptapActiveState,
}

impl TiptapSelectionState {
    /// Returns all boolean states contributed by extensions selected for the editor.
    #[must_use]
    pub const fn active_state(&self) -> &TiptapActiveState {
        &self.active
    }

    /// Returns the reported value for `key`, or `None` when no selected extension contributes it.
    #[must_use]
    pub fn active(&self, key: TiptapActiveKey) -> Option<bool> {
        self.active.get(key)
    }

    /// Returns whether `key` is reported as active.
    ///
    /// Missing keys are treated as inactive. Use [`Self::active`] when the distinction between
    /// missing and explicitly inactive matters.
    #[must_use]
    pub fn is_active(&self, key: TiptapActiveKey) -> bool {
        self.active.is_active(key)
    }

    /// Returns all contributed active-state entries in stable key order.
    #[must_use]
    pub fn active_entries(
        &self,
    ) -> impl ExactSizeIterator<Item = (TiptapActiveKey, bool)> + DoubleEndedIterator + '_ {
        self.active.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;
    use serde_json::json;

    #[test]
    fn deserializes_missing_active_state_as_empty() {
        let state: TiptapSelectionState = serde_json::from_value(json!({})).unwrap();

        assert_that!(state).is_equal_to(TiptapSelectionState::default());
    }

    #[test]
    fn distinguishes_active_inactive_and_missing_keys() {
        let state: TiptapSelectionState = serde_json::from_value(json!({
            "active": {
                "bold": true,
                "italic": false,
            },
        }))
        .unwrap();

        assert_that!(state.active(TiptapActiveKey::Bold)).is_equal_to(Some(true));
        assert_that!(state.is_active(TiptapActiveKey::Bold)).is_true();
        assert_that!(state.active(TiptapActiveKey::Italic)).is_equal_to(Some(false));
        assert_that!(state.is_active(TiptapActiveKey::Italic)).is_false();
        assert_that!(state.active(TiptapActiveKey::Strike)).is_equal_to(None);
        assert_that!(state.is_active(TiptapActiveKey::Strike)).is_false();
    }

    #[test]
    fn serializes_nested_active_state() {
        let state: TiptapSelectionState = serde_json::from_value(json!({
            "active": {"bold": true},
        }))
        .unwrap();

        assert_that!(serde_json::to_value(state).unwrap())
            .is_equal_to(json!({"active": {"bold": true}}));
    }

    #[test]
    fn rejects_unknown_active_keys() {
        let result = serde_json::from_value::<TiptapSelectionState>(json!({
            "active": {"not_an_active_key": true},
        }));

        assert_that!(result.is_err()).is_true();
    }

    #[test]
    fn rejects_unknown_selection_state_fields() {
        let result = serde_json::from_value::<TiptapSelectionState>(json!({
            "active": {},
            "not_selection_state": true,
        }));

        assert_that!(result.is_err()).is_true();
    }
}
