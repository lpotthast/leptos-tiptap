use crate::TiptapEditorError;
#[cfg(not(feature = "ssr"))]
use std::sync::OnceLock;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsValue;

#[cfg(not(feature = "ssr"))]
use super::ffi;

#[cfg(not(feature = "ssr"))]
static RUNTIME_INITIALIZED: OnceLock<Result<(), TiptapEditorError>> = OnceLock::new();

#[cfg(not(feature = "ssr"))]
static COMPILED_EXTENSIONS_REGISTERED: OnceLock<Result<(), TiptapEditorError>> = OnceLock::new();

#[cfg(not(feature = "ssr"))]
fn format_js_value(value: JsValue) -> String {
    value.as_string().unwrap_or_else(|| format!("{value:?}"))
}

#[cfg(not(feature = "ssr"))]
fn map_registration_error(name: &str, value: JsValue) -> TiptapEditorError {
    TiptapEditorError::BridgeError(format!(
        "could not register Tiptap extension '{name}': {}",
        format_js_value(value),
    ))
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
fn register_extension(
    name: &str,
    register: impl FnOnce() -> Result<(), JsValue>,
) -> Result<(), TiptapEditorError> {
    register().map_err(|value| map_registration_error(name, value))
}

#[cfg(not(feature = "ssr"))]
fn ensure_runtime_initialized() -> Result<(), TiptapEditorError> {
    RUNTIME_INITIALIZED
        .get_or_init(|| {
            ffi::init_bridge_runtime()
                .map_err(|value| map_registration_error("bridge_runtime", value))
        })
        .clone()
}

#[cfg(not(feature = "ssr"))]
fn register_compiled_extensions() -> Result<(), TiptapEditorError> {
    ensure_runtime_initialized()?;

    #[cfg(feature = "blockquote")]
    register_extension("blockquote", ffi::register_blockquote)?;
    #[cfg(feature = "bold")]
    register_extension("bold", ffi::register_bold)?;
    #[cfg(feature = "bullet_list")]
    register_extension("bullet_list", ffi::register_bullet_list)?;
    #[cfg(feature = "code")]
    register_extension("code", ffi::register_code)?;
    #[cfg(feature = "code_block")]
    register_extension("code_block", ffi::register_code_block)?;
    #[cfg(feature = "document")]
    register_extension("document", ffi::register_document)?;
    #[cfg(feature = "dropcursor")]
    register_extension("dropcursor", ffi::register_dropcursor)?;
    #[cfg(feature = "gapcursor")]
    register_extension("gapcursor", ffi::register_gapcursor)?;
    #[cfg(feature = "hard_break")]
    register_extension("hard_break", ffi::register_hard_break)?;
    #[cfg(feature = "heading")]
    register_extension("heading", ffi::register_heading)?;
    #[cfg(feature = "history")]
    register_extension("history", ffi::register_history)?;
    #[cfg(feature = "horizontal_rule")]
    register_extension("horizontal_rule", ffi::register_horizontal_rule)?;
    #[cfg(feature = "italic")]
    register_extension("italic", ffi::register_italic)?;
    #[cfg(feature = "list_item")]
    register_extension("list_item", ffi::register_list_item)?;
    #[cfg(feature = "ordered_list")]
    register_extension("ordered_list", ffi::register_ordered_list)?;
    #[cfg(feature = "paragraph")]
    register_extension("paragraph", ffi::register_paragraph)?;
    #[cfg(feature = "strike")]
    register_extension("strike", ffi::register_strike)?;
    #[cfg(feature = "text")]
    register_extension("text", ffi::register_text)?;
    #[cfg(feature = "text_align")]
    register_extension("text_align", ffi::register_text_align)?;
    #[cfg(feature = "highlight")]
    register_extension("highlight", ffi::register_highlight)?;
    #[cfg(feature = "image")]
    register_extension("image", ffi::register_image)?;
    #[cfg(feature = "link")]
    register_extension("link", ffi::register_link)?;
    #[cfg(feature = "youtube")]
    register_extension("youtube", ffi::register_youtube)?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub(crate) fn ensure_compiled_extensions_registered() -> Result<(), TiptapEditorError> {
    COMPILED_EXTENSIONS_REGISTERED
        .get_or_init(register_compiled_extensions)
        .clone()
}

#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub(crate) fn ensure_compiled_extensions_registered() -> Result<(), TiptapEditorError> {
    Ok(())
}
