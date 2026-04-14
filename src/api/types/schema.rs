use serde::{Deserialize, Serialize};

/// A node or mark target for schema-based commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapSchemaTarget {
    /// A node type target.
    Node(TiptapNodeName),
    /// A mark type target.
    Mark(TiptapMarkName),
}

impl TiptapSchemaTarget {
    pub(crate) fn schema_name(self) -> &'static str {
        match self {
            Self::Node(node) => node.schema_name(),
            Self::Mark(mark) => mark.schema_name(),
        }
    }
}

/// Tiptap node schema names supported by enabled Cargo features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TiptapNodeName {
    #[cfg(feature = "blockquote")]
    #[serde(rename = "blockquote")]
    /// The `blockquote` node.
    Blockquote,
    #[cfg(feature = "bullet_list")]
    #[serde(rename = "bulletList")]
    /// The `bulletList` node.
    BulletList,
    #[cfg(feature = "code_block")]
    #[serde(rename = "codeBlock")]
    /// The `codeBlock` node.
    CodeBlock,
    #[cfg(feature = "document")]
    #[serde(rename = "doc")]
    /// The `doc` node.
    Doc,
    #[cfg(feature = "hard_break")]
    #[serde(rename = "hardBreak")]
    /// The `hardBreak` node.
    HardBreak,
    #[cfg(feature = "heading")]
    #[serde(rename = "heading")]
    /// The `heading` node.
    Heading,
    #[cfg(feature = "horizontal_rule")]
    #[serde(rename = "horizontalRule")]
    /// The `horizontalRule` node.
    HorizontalRule,
    #[cfg(feature = "image")]
    #[serde(rename = "image")]
    /// The `image` node.
    Image,
    #[cfg(feature = "list_item")]
    #[serde(rename = "listItem")]
    /// The `listItem` node.
    ListItem,
    #[cfg(feature = "ordered_list")]
    #[serde(rename = "orderedList")]
    /// The `orderedList` node.
    OrderedList,
    #[cfg(feature = "paragraph")]
    #[serde(rename = "paragraph")]
    /// The `paragraph` node.
    Paragraph,
    #[cfg(feature = "text")]
    #[serde(rename = "text")]
    /// The `text` node.
    Text,
    #[cfg(feature = "youtube")]
    #[serde(rename = "youtube")]
    /// The `youtube` node.
    Youtube,
}

impl TiptapNodeName {
    pub(crate) fn schema_name(self) -> &'static str {
        match self {
            #[cfg(feature = "blockquote")]
            Self::Blockquote => "blockquote",
            #[cfg(feature = "bullet_list")]
            Self::BulletList => "bulletList",
            #[cfg(feature = "code_block")]
            Self::CodeBlock => "codeBlock",
            #[cfg(feature = "document")]
            Self::Doc => "doc",
            #[cfg(feature = "hard_break")]
            Self::HardBreak => "hardBreak",
            #[cfg(feature = "heading")]
            Self::Heading => "heading",
            #[cfg(feature = "horizontal_rule")]
            Self::HorizontalRule => "horizontalRule",
            #[cfg(feature = "image")]
            Self::Image => "image",
            #[cfg(feature = "list_item")]
            Self::ListItem => "listItem",
            #[cfg(feature = "ordered_list")]
            Self::OrderedList => "orderedList",
            #[cfg(feature = "paragraph")]
            Self::Paragraph => "paragraph",
            #[cfg(feature = "text")]
            Self::Text => "text",
            #[cfg(feature = "youtube")]
            Self::Youtube => "youtube",
        }
    }
}

/// Tiptap mark schema names supported by enabled Cargo features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TiptapMarkName {
    #[cfg(feature = "bold")]
    #[serde(rename = "bold")]
    /// The `bold` mark.
    Bold,
    #[cfg(feature = "code")]
    #[serde(rename = "code")]
    /// The `code` mark.
    Code,
    #[cfg(feature = "highlight")]
    #[serde(rename = "highlight")]
    /// The `highlight` mark.
    Highlight,
    #[cfg(feature = "italic")]
    #[serde(rename = "italic")]
    /// The `italic` mark.
    Italic,
    #[cfg(feature = "link")]
    #[serde(rename = "link")]
    /// The `link` mark.
    Link,
    #[cfg(feature = "strike")]
    #[serde(rename = "strike")]
    /// The `strike` mark.
    Strike,
}

impl TiptapMarkName {
    pub(crate) fn schema_name(self) -> &'static str {
        match self {
            #[cfg(feature = "bold")]
            Self::Bold => "bold",
            #[cfg(feature = "code")]
            Self::Code => "code",
            #[cfg(feature = "highlight")]
            Self::Highlight => "highlight",
            #[cfg(feature = "italic")]
            Self::Italic => "italic",
            #[cfg(feature = "link")]
            Self::Link => "link",
            #[cfg(feature = "strike")]
            Self::Strike => "strike",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;
    use serde_json::json;

    #[cfg(feature = "document")]
    #[test]
    fn serializes_node_names_to_exact_schema_names() {
        assert_that!(serde_json::to_value(TiptapNodeName::Doc).unwrap()).is_equal_to(json!("doc"));
    }

    #[cfg(feature = "bold")]
    #[test]
    fn serializes_mark_names_to_exact_schema_names() {
        assert_that!(serde_json::to_value(TiptapMarkName::Bold).unwrap())
            .is_equal_to(json!("bold"));
    }
}
