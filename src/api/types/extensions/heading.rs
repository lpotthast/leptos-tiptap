/// Heading levels supported by Tiptap's heading extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapHeadingLevel {
    /// Level-1 heading.
    H1,
    /// Level-2 heading.
    H2,
    /// Level-3 heading.
    H3,
    /// Level-4 heading.
    H4,
    /// Level-5 heading.
    H5,
    /// Level-6 heading.
    H6,
}

impl From<TiptapHeadingLevel> for i32 {
    fn from(val: TiptapHeadingLevel) -> Self {
        match val {
            TiptapHeadingLevel::H1 => 1,
            TiptapHeadingLevel::H2 => 2,
            TiptapHeadingLevel::H3 => 3,
            TiptapHeadingLevel::H4 => 4,
            TiptapHeadingLevel::H5 => 5,
            TiptapHeadingLevel::H6 => 6,
        }
    }
}
