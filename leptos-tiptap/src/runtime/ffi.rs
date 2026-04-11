#[cfg(not(feature = "ssr"))]
mod js {
    use wasm_bindgen::closure::ScopedClosure;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/js/generated/bridge_runtime.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn init_bridge_runtime() -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub fn create(
            request: JsValue,
            on_ready: &ScopedClosure<'static, dyn Fn(JsValue)>,
            on_change: &ScopedClosure<'static, dyn Fn()>,
            on_selection: &ScopedClosure<'static, dyn Fn(JsValue)>,
            on_error: &ScopedClosure<'static, dyn Fn(JsValue)>,
        ) -> Result<(), JsValue>;
        pub fn destroy(id: String);
        pub fn command(request: JsValue) -> JsValue;
        pub fn document(request: JsValue) -> JsValue;
    }

    #[cfg(feature = "blockquote")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_blockquote.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_blockquote() -> Result<(), JsValue>;
    }

    #[cfg(feature = "bold")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_bold.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_bold() -> Result<(), JsValue>;
    }

    #[cfg(feature = "bullet_list")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_bullet_list.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_bullet_list() -> Result<(), JsValue>;
    }

    #[cfg(feature = "code")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_code.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_code() -> Result<(), JsValue>;
    }

    #[cfg(feature = "code_block")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_code_block.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_code_block() -> Result<(), JsValue>;
    }

    #[cfg(feature = "document")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_document.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_document() -> Result<(), JsValue>;
    }

    #[cfg(feature = "dropcursor")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_dropcursor.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_dropcursor() -> Result<(), JsValue>;
    }

    #[cfg(feature = "gapcursor")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_gapcursor.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_gapcursor() -> Result<(), JsValue>;
    }

    #[cfg(feature = "hard_break")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_hard_break.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_hard_break() -> Result<(), JsValue>;
    }

    #[cfg(feature = "heading")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_heading.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_heading() -> Result<(), JsValue>;
    }

    #[cfg(feature = "history")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_history.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_history() -> Result<(), JsValue>;
    }

    #[cfg(feature = "horizontal_rule")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_horizontal_rule.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_horizontal_rule() -> Result<(), JsValue>;
    }

    #[cfg(feature = "italic")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_italic.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_italic() -> Result<(), JsValue>;
    }

    #[cfg(feature = "list_item")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_list_item.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_list_item() -> Result<(), JsValue>;
    }

    #[cfg(feature = "ordered_list")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_ordered_list.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_ordered_list() -> Result<(), JsValue>;
    }

    #[cfg(feature = "paragraph")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_paragraph.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_paragraph() -> Result<(), JsValue>;
    }

    #[cfg(feature = "strike")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_strike.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_strike() -> Result<(), JsValue>;
    }

    #[cfg(feature = "text")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_text.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_text() -> Result<(), JsValue>;
    }

    #[cfg(feature = "text_align")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_text_align.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_text_align() -> Result<(), JsValue>;
    }

    #[cfg(feature = "highlight")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_highlight.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_highlight() -> Result<(), JsValue>;
    }

    #[cfg(feature = "image")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_image.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_image() -> Result<(), JsValue>;
    }

    #[cfg(feature = "link")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_link.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_link() -> Result<(), JsValue>;
    }

    #[cfg(feature = "youtube")]
    #[wasm_bindgen(module = "/src/js/generated/tiptap_youtube.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_youtube() -> Result<(), JsValue>;
    }
}

#[cfg(not(feature = "ssr"))]
pub(crate) use js::{command, create, destroy, document, init_bridge_runtime};

#[cfg(all(not(feature = "ssr"), feature = "blockquote"))]
pub(crate) use js::register_blockquote;
#[cfg(all(not(feature = "ssr"), feature = "bold"))]
pub(crate) use js::register_bold;
#[cfg(all(not(feature = "ssr"), feature = "bullet_list"))]
pub(crate) use js::register_bullet_list;
#[cfg(all(not(feature = "ssr"), feature = "code"))]
pub(crate) use js::register_code;
#[cfg(all(not(feature = "ssr"), feature = "code_block"))]
pub(crate) use js::register_code_block;
#[cfg(all(not(feature = "ssr"), feature = "document"))]
pub(crate) use js::register_document;
#[cfg(all(not(feature = "ssr"), feature = "dropcursor"))]
pub(crate) use js::register_dropcursor;
#[cfg(all(not(feature = "ssr"), feature = "gapcursor"))]
pub(crate) use js::register_gapcursor;
#[cfg(all(not(feature = "ssr"), feature = "hard_break"))]
pub(crate) use js::register_hard_break;
#[cfg(all(not(feature = "ssr"), feature = "heading"))]
pub(crate) use js::register_heading;
#[cfg(all(not(feature = "ssr"), feature = "highlight"))]
pub(crate) use js::register_highlight;
#[cfg(all(not(feature = "ssr"), feature = "history"))]
pub(crate) use js::register_history;
#[cfg(all(not(feature = "ssr"), feature = "horizontal_rule"))]
pub(crate) use js::register_horizontal_rule;
#[cfg(all(not(feature = "ssr"), feature = "image"))]
pub(crate) use js::register_image;
#[cfg(all(not(feature = "ssr"), feature = "italic"))]
pub(crate) use js::register_italic;
#[cfg(all(not(feature = "ssr"), feature = "link"))]
pub(crate) use js::register_link;
#[cfg(all(not(feature = "ssr"), feature = "list_item"))]
pub(crate) use js::register_list_item;
#[cfg(all(not(feature = "ssr"), feature = "ordered_list"))]
pub(crate) use js::register_ordered_list;
#[cfg(all(not(feature = "ssr"), feature = "paragraph"))]
pub(crate) use js::register_paragraph;
#[cfg(all(not(feature = "ssr"), feature = "strike"))]
pub(crate) use js::register_strike;
#[cfg(all(not(feature = "ssr"), feature = "text"))]
pub(crate) use js::register_text;
#[cfg(all(not(feature = "ssr"), feature = "text_align"))]
pub(crate) use js::register_text_align;
#[cfg(all(not(feature = "ssr"), feature = "youtube"))]
pub(crate) use js::register_youtube;
