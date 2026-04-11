use super::TiptapEditorError;

/// A Tiptap extension that can be compiled into the runtime and activated per editor instance.
///
/// The available variants depend on the enabled Cargo features for `leptos-tiptap`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TiptapExtension {
    #[cfg(feature = "blockquote")]
    Blockquote,
    #[cfg(feature = "bold")]
    Bold,
    #[cfg(feature = "bullet_list")]
    BulletList,
    #[cfg(feature = "code")]
    Code,
    #[cfg(feature = "code_block")]
    CodeBlock,
    #[cfg(feature = "document")]
    Document,
    #[cfg(feature = "dropcursor")]
    Dropcursor,
    #[cfg(feature = "gapcursor")]
    Gapcursor,
    #[cfg(feature = "hard_break")]
    HardBreak,
    #[cfg(feature = "heading")]
    Heading,
    #[cfg(feature = "history")]
    History,
    #[cfg(feature = "horizontal_rule")]
    HorizontalRule,
    #[cfg(feature = "italic")]
    Italic,
    #[cfg(feature = "list_item")]
    ListItem,
    #[cfg(feature = "ordered_list")]
    OrderedList,
    #[cfg(feature = "paragraph")]
    Paragraph,
    #[cfg(feature = "strike")]
    Strike,
    #[cfg(feature = "text")]
    Text,
    #[cfg(feature = "text_align")]
    TextAlign,
    #[cfg(feature = "highlight")]
    Highlight,
    #[cfg(feature = "image")]
    Image,
    #[cfg(feature = "link")]
    Link,
    #[cfg(feature = "youtube")]
    Youtube,
}

impl TiptapExtension {
    pub fn name(self) -> &'static str {
        match self {
            #[cfg(feature = "blockquote")]
            Self::Blockquote => "blockquote",
            #[cfg(feature = "bold")]
            Self::Bold => "bold",
            #[cfg(feature = "bullet_list")]
            Self::BulletList => "bullet_list",
            #[cfg(feature = "code")]
            Self::Code => "code",
            #[cfg(feature = "code_block")]
            Self::CodeBlock => "code_block",
            #[cfg(feature = "document")]
            Self::Document => "document",
            #[cfg(feature = "dropcursor")]
            Self::Dropcursor => "dropcursor",
            #[cfg(feature = "gapcursor")]
            Self::Gapcursor => "gapcursor",
            #[cfg(feature = "hard_break")]
            Self::HardBreak => "hard_break",
            #[cfg(feature = "heading")]
            Self::Heading => "heading",
            #[cfg(feature = "history")]
            Self::History => "history",
            #[cfg(feature = "horizontal_rule")]
            Self::HorizontalRule => "horizontal_rule",
            #[cfg(feature = "italic")]
            Self::Italic => "italic",
            #[cfg(feature = "list_item")]
            Self::ListItem => "list_item",
            #[cfg(feature = "ordered_list")]
            Self::OrderedList => "ordered_list",
            #[cfg(feature = "paragraph")]
            Self::Paragraph => "paragraph",
            #[cfg(feature = "strike")]
            Self::Strike => "strike",
            #[cfg(feature = "text")]
            Self::Text => "text",
            #[cfg(feature = "text_align")]
            Self::TextAlign => "text_align",
            #[cfg(feature = "highlight")]
            Self::Highlight => "highlight",
            #[cfg(feature = "image")]
            Self::Image => "image",
            #[cfg(feature = "link")]
            Self::Link => "link",
            #[cfg(feature = "youtube")]
            Self::Youtube => "youtube",
        }
    }

    pub fn all_enabled() -> Vec<Self> {
        vec![
            #[cfg(feature = "blockquote")]
            Self::Blockquote,
            #[cfg(feature = "bold")]
            Self::Bold,
            #[cfg(feature = "bullet_list")]
            Self::BulletList,
            #[cfg(feature = "code")]
            Self::Code,
            #[cfg(feature = "code_block")]
            Self::CodeBlock,
            #[cfg(feature = "document")]
            Self::Document,
            #[cfg(feature = "dropcursor")]
            Self::Dropcursor,
            #[cfg(feature = "gapcursor")]
            Self::Gapcursor,
            #[cfg(feature = "hard_break")]
            Self::HardBreak,
            #[cfg(feature = "heading")]
            Self::Heading,
            #[cfg(feature = "history")]
            Self::History,
            #[cfg(feature = "horizontal_rule")]
            Self::HorizontalRule,
            #[cfg(feature = "italic")]
            Self::Italic,
            #[cfg(feature = "list_item")]
            Self::ListItem,
            #[cfg(feature = "ordered_list")]
            Self::OrderedList,
            #[cfg(feature = "paragraph")]
            Self::Paragraph,
            #[cfg(feature = "strike")]
            Self::Strike,
            #[cfg(feature = "text")]
            Self::Text,
            #[cfg(feature = "text_align")]
            Self::TextAlign,
            #[cfg(feature = "highlight")]
            Self::Highlight,
            #[cfg(feature = "image")]
            Self::Image,
            #[cfg(feature = "link")]
            Self::Link,
            #[cfg(feature = "youtube")]
            Self::Youtube,
        ]
    }

    #[cfg(not(feature = "ssr"))]
    #[allow(dead_code)]
    pub(crate) fn js_name(self) -> &'static str {
        self.name()
    }

    #[cfg_attr(feature = "ssr", allow(dead_code))]
    pub(crate) fn validate_extension_set(extensions: &[Self]) -> Result<(), TiptapEditorError> {
        let mut missing = Vec::new();

        for &extension in extensions {
            extension.collect_missing_dependencies(extensions, &mut missing);
        }

        missing.sort_unstable();
        missing.dedup();

        if missing.is_empty() {
            return Ok(());
        }

        Err(TiptapEditorError::BridgeError(format!(
            "invalid Tiptap extension set: missing required extension(s): {}",
            missing.join(", ")
        )))
    }

    #[allow(unused_macros, unused_variables)]
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    fn collect_missing_dependencies(self, selected: &[Self], missing: &mut Vec<&'static str>) {
        macro_rules! require {
            ($dependency:expr) => {
                if !selected.contains(&$dependency) {
                    missing.push($dependency.name());
                }
            };
        }

        match self {
            #[cfg(feature = "bullet_list")]
            Self::BulletList => {
                require!(Self::ListItem);
            }
            #[cfg(feature = "ordered_list")]
            Self::OrderedList => {
                require!(Self::ListItem);
            }
            #[cfg(feature = "text_align")]
            Self::TextAlign => {
                require!(Self::Heading);
                require!(Self::Paragraph);
            }
            #[allow(unreachable_patterns)]
            _ => {}
        }
    }
}

#[cfg(all(
    test,
    any(
        all(feature = "bullet_list", feature = "list_item"),
        all(feature = "text_align", feature = "heading", feature = "paragraph")
    )
))]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[cfg(all(feature = "bullet_list", feature = "list_item"))]
    #[test]
    fn validates_list_item_dependencies() {
        let error =
            TiptapExtension::validate_extension_set(&[TiptapExtension::BulletList]).unwrap_err();

        assert_that!(error.to_string()).contains("list_item");
    }

    #[cfg(all(feature = "text_align", feature = "heading", feature = "paragraph"))]
    #[test]
    fn validates_text_align_dependencies() {
        let error =
            TiptapExtension::validate_extension_set(&[TiptapExtension::TextAlign]).unwrap_err();

        assert_that!(error.to_string()).contains("heading");
        assert_that!(error.to_string()).contains("paragraph");
    }
}
