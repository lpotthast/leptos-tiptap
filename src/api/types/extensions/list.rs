use serde::{Deserialize, Serialize};

use super::super::shared::TiptapAttributes;

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

/// Options for toggling list nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TiptapToggleListOptions {
    /// Whether active marks should be kept on the new list item.
    pub keep_marks: Option<bool>,
    /// Optional attributes to apply to the list.
    pub attributes: Option<TiptapAttributes>,
}
