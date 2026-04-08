/// The compiled TipTap and Rust-bridge JS module.
///
/// Can be included in a site using `<link rel="modulepreload" href="/js/tiptap.js" />` once stored
/// in your project as '/js/tiptap.js'.
pub const TIPTAP_JS: &str = include_str!("../dist/tiptap.js");
