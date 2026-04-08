use crate::TiptapEditorError;
use cfg_if::cfg_if;
use serde::Deserialize;
#[cfg(not(feature = "ssr"))]
use serde::{de::DeserializeOwned, Serialize};
#[cfg(not(feature = "ssr"))]
use std::sync::OnceLock;
use wasm_bindgen::closure::ScopedClosure;
use wasm_bindgen::JsValue;

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(feature = "ssr"), serde(rename_all = "snake_case"))]
pub(crate) enum ContentFormat {
    Html,
    Json,
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContentPayload {
    pub(crate) format: ContentFormat,
    pub(crate) value: String,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct CreateRequest {
    id: String,
    content: ContentPayload,
    editable: bool,
    extensions: Vec<&'static str>,
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub(crate) struct ReadyPayload {
    pub(crate) generation: u32,
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "ssr"), serde(tag = "kind", rename_all = "snake_case"))]
pub(crate) enum EditorCommand {
    ToggleHeading {
        level: i32,
    },
    SetParagraph,
    ToggleBold,
    ToggleItalic,
    ToggleStrike,
    ToggleBlockquote,
    ToggleHighlight,
    ToggleBulletList,
    ToggleOrderedList,
    SetTextAlignLeft,
    SetTextAlignCenter,
    SetTextAlignRight,
    SetTextAlignJustify,
    SetImage {
        src: String,
        alt: String,
        title: String,
    },
    SetLink {
        href: String,
        target: Option<String>,
        rel: Option<String>,
    },
    ToggleLink {
        href: String,
        target: Option<String>,
        rel: Option<String>,
    },
    UnsetLink,
    SetYoutubeVideo {
        src: String,
        start: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
    },
    SetEditable {
        editable: bool,
    },
}

impl EditorCommand {
    #[cfg(not(feature = "ssr"))]
    fn operation_name(&self) -> &'static str {
        match self {
            Self::ToggleHeading { .. } => "toggle_heading",
            Self::SetParagraph => "set_paragraph",
            Self::ToggleBold => "toggle_bold",
            Self::ToggleItalic => "toggle_italic",
            Self::ToggleStrike => "toggle_strike",
            Self::ToggleBlockquote => "toggle_blockquote",
            Self::ToggleHighlight => "toggle_highlight",
            Self::ToggleBulletList => "toggle_bullet_list",
            Self::ToggleOrderedList => "toggle_ordered_list",
            Self::SetTextAlignLeft => "set_text_align_left",
            Self::SetTextAlignCenter => "set_text_align_center",
            Self::SetTextAlignRight => "set_text_align_right",
            Self::SetTextAlignJustify => "set_text_align_justify",
            Self::SetImage { .. } => "set_image",
            Self::SetLink { .. } => "set_link",
            Self::ToggleLink { .. } => "toggle_link",
            Self::UnsetLink => "unset_link",
            Self::SetYoutubeVideo { .. } => "set_youtube_video",
            Self::SetEditable { .. } => "set_editable",
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "ssr"), serde(tag = "kind", rename_all = "snake_case"))]
pub(crate) enum DocumentRequest {
    GetContent { format: ContentFormat },
    SetContent { content: ContentPayload },
}

impl DocumentRequest {
    #[cfg(not(feature = "ssr"))]
    fn operation_name(&self) -> &'static str {
        match self {
            Self::GetContent {
                format: ContentFormat::Html,
            } => "get_content_html",
            Self::GetContent {
                format: ContentFormat::Json,
            } => "get_content_json",
            Self::SetContent { .. } => "set_content",
        }
    }
}

#[cfg_attr(not(feature = "ssr"), derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
#[cfg_attr(not(feature = "ssr"), serde(tag = "kind", rename_all = "snake_case"))]
pub(crate) enum DocumentResponse {
    Content { content: ContentPayload },
    Empty,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Serialize)]
struct CommandInvocation {
    id: String,
    generation: u32,
    command: EditorCommand,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Serialize)]
struct DocumentInvocation {
    id: String,
    generation: u32,
    request: DocumentRequest,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Deserialize)]
struct JsInteropError {
    kind: String,
    message: String,
    operation: Option<String>,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Deserialize)]
struct JsInteropResponse<T> {
    ok: bool,
    value: Option<T>,
    error: Option<JsInteropError>,
}

#[cfg(not(feature = "ssr"))]
mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/js/generated/tiptap_core.js")]
    extern "C" {
        pub fn init_tiptap_core();
    }

    #[wasm_bindgen(module = "/src/js/generated/bridge_runtime.js")]
    extern "C" {
        pub fn init_bridge_runtime();
        pub fn create(
            request: JsValue,
            onReady: &ScopedClosure<'static, dyn Fn(JsValue)>,
            onChange: &ScopedClosure<'static, dyn Fn()>,
            onSelection: &ScopedClosure<'static, dyn Fn(JsValue)>,
            onError: &ScopedClosure<'static, dyn Fn(JsValue)>,
        );
        pub fn destroy(id: String);
        pub fn command(request: JsValue) -> JsValue;
        pub fn document(request: JsValue) -> JsValue;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_blockquote.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_blockquote() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_bold.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_bold() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_bullet_list.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_bullet_list() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_code.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_code() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_code_block.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_code_block() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_document.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_document() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_dropcursor.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_dropcursor() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_gapcursor.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_gapcursor() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_hard_break.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_hard_break() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_heading.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_heading() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_history.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_history() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_horizontal_rule.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_horizontal_rule() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_italic.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_italic() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_list_item.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_list_item() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_ordered_list.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_ordered_list() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_paragraph.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_paragraph() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_strike.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_strike() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_text.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_text() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_text_align.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_text_align() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_highlight.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_highlight() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_image.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_image() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_link.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_link() -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/js/generated/tiptap_youtube.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn register_youtube() -> Result<(), JsValue>;
    }
}

#[cfg(not(feature = "ssr"))]
fn serialize_request<T: Serialize>(request: &T) -> Result<JsValue, TiptapEditorError> {
    serde_wasm_bindgen::to_value(request).map_err(|err| {
        TiptapEditorError::BridgeError(format!("could not serialize JS bridge request: {err}"))
    })
}

#[cfg(not(feature = "ssr"))]
fn deserialize_response<T: DeserializeOwned>(
    value: JsValue,
) -> Result<JsInteropResponse<T>, TiptapEditorError> {
    serde_wasm_bindgen::from_value(value).map_err(|err| {
        TiptapEditorError::BridgeError(format!("could not deserialize JS bridge response: {err}"))
    })
}

#[cfg(not(feature = "ssr"))]
fn deserialize_error(value: JsValue) -> Result<JsInteropError, TiptapEditorError> {
    serde_wasm_bindgen::from_value(value).map_err(|err| {
        TiptapEditorError::BridgeError(format!("could not deserialize JS bridge error: {err}"))
    })
}

#[cfg(not(feature = "ssr"))]
const DEFAULT_EXTENSION_NAMES: &[&str] = &[
    "blockquote",
    "bold",
    "bullet_list",
    "code",
    "code_block",
    "document",
    "dropcursor",
    "gapcursor",
    "hard_break",
    "heading",
    "history",
    "horizontal_rule",
    "italic",
    "list_item",
    "ordered_list",
    "paragraph",
    "strike",
    "text",
    "text_align",
    "highlight",
    "image",
    "link",
    "youtube",
];

#[cfg(not(feature = "ssr"))]
static DEFAULT_EXTENSIONS_REGISTERED: OnceLock<Result<(), TiptapEditorError>> = OnceLock::new();

#[cfg(not(feature = "ssr"))]
fn map_bridge_error(error: JsInteropError) -> TiptapEditorError {
    let JsInteropError {
        kind,
        message,
        operation,
    } = error;

    match kind.as_str() {
        "editor_unavailable" => TiptapEditorError::EditorUnavailable,
        "editor_mount_failed" => TiptapEditorError::MountFailed(message),
        "invalid_content" => TiptapEditorError::InvalidContent(message),
        "extension_unavailable" => TiptapEditorError::OperationFailed {
            operation: operation.unwrap_or_else(|| "unknown".to_owned()),
            message,
        },
        "extension_registration_failed" => TiptapEditorError::BridgeError(message),
        "command_rejected" => TiptapEditorError::CommandRejected {
            operation: operation.unwrap_or_else(|| "unknown".to_owned()),
            message,
        },
        "operation_failed" => TiptapEditorError::OperationFailed {
            operation: operation.unwrap_or_else(|| "unknown".to_owned()),
            message,
        },
        other => TiptapEditorError::BridgeError(format!(
            "received unknown JS bridge error kind '{other}'",
        )),
    }
}

#[cfg(not(feature = "ssr"))]
fn response_to_result<T>(response: JsInteropResponse<T>) -> Result<T, TiptapEditorError> {
    match response {
        JsInteropResponse {
            ok: true,
            value: Some(value),
            ..
        } => Ok(value),
        JsInteropResponse {
            ok: true,
            value: None,
            ..
        } => Err(TiptapEditorError::BridgeError(
            "received an ok JS bridge response without a value".to_owned(),
        )),
        JsInteropResponse {
            ok: false,
            error: Some(error),
            ..
        } => Err(map_bridge_error(error)),
        JsInteropResponse {
            ok: false,
            error: None,
            ..
        } => Err(TiptapEditorError::BridgeError(
            "received an error JS bridge response without error details".to_owned(),
        )),
    }
}

#[cfg(not(feature = "ssr"))]
fn invoke_bridge<TRequest, TResponse>(
    request: &TRequest,
    invoke: impl FnOnce(JsValue) -> JsValue,
) -> Result<TResponse, TiptapEditorError>
where
    TRequest: Serialize,
    TResponse: DeserializeOwned,
{
    let request = serialize_request(request)?;
    let response = invoke(request);
    response_to_result(deserialize_response(response)?)
}

#[cfg(not(feature = "ssr"))]
fn map_registration_error(name: &str, value: JsValue) -> TiptapEditorError {
    let message = value
        .as_string()
        .unwrap_or_else(|| format!("{value:?}"));

    TiptapEditorError::BridgeError(format!(
        "could not register Tiptap extension '{name}': {message}"
    ))
}

#[cfg(not(feature = "ssr"))]
fn register_extension(
    name: &str,
    register: impl FnOnce() -> Result<(), JsValue>,
) -> Result<(), TiptapEditorError> {
    register().map_err(|value| map_registration_error(name, value))
}

#[cfg(not(feature = "ssr"))]
fn register_default_extensions() -> Result<(), TiptapEditorError> {
    js::init_tiptap_core();
    js::init_bridge_runtime();

    register_extension("blockquote", js::register_blockquote)?;
    register_extension("bold", js::register_bold)?;
    register_extension("bullet_list", js::register_bullet_list)?;
    register_extension("code", js::register_code)?;
    register_extension("code_block", js::register_code_block)?;
    register_extension("document", js::register_document)?;
    register_extension("dropcursor", js::register_dropcursor)?;
    register_extension("gapcursor", js::register_gapcursor)?;
    register_extension("hard_break", js::register_hard_break)?;
    register_extension("heading", js::register_heading)?;
    register_extension("history", js::register_history)?;
    register_extension("horizontal_rule", js::register_horizontal_rule)?;
    register_extension("italic", js::register_italic)?;
    register_extension("list_item", js::register_list_item)?;
    register_extension("ordered_list", js::register_ordered_list)?;
    register_extension("paragraph", js::register_paragraph)?;
    register_extension("strike", js::register_strike)?;
    register_extension("text", js::register_text)?;
    register_extension("text_align", js::register_text_align)?;
    register_extension("highlight", js::register_highlight)?;
    register_extension("image", js::register_image)?;
    register_extension("link", js::register_link)?;
    register_extension("youtube", js::register_youtube)?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn ensure_default_extensions_registered() -> Result<(), TiptapEditorError> {
    DEFAULT_EXTENSIONS_REGISTERED
        .get_or_init(register_default_extensions)
        .clone()
}

pub(crate) fn error_from_js_value(value: JsValue) -> TiptapEditorError {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        match deserialize_error(value) {
            Ok(error) => map_bridge_error(error),
            Err(err) => err,
        }
    } else {
        let _value = value;
        TiptapEditorError::BridgeError(
            "received a JS error callback while compiling for SSR".to_owned(),
        )
    }}
}

pub(crate) fn create(
    id: String,
    content: ContentPayload,
    editable: bool,
    on_ready: &ScopedClosure<'static, dyn Fn(JsValue)>,
    on_change: &ScopedClosure<'static, dyn Fn()>,
    on_selection: &ScopedClosure<'static, dyn Fn(JsValue)>,
    on_error: &ScopedClosure<'static, dyn Fn(JsValue)>,
) -> Result<(), TiptapEditorError> {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        ensure_default_extensions_registered()?;

        let request = serialize_request(&CreateRequest {
            id,
            content,
            editable,
            extensions: DEFAULT_EXTENSION_NAMES.to_vec(),
        })
        .expect("serializing the Tiptap create request should not fail");

        js::create(
            request,
            on_ready,
            on_change,
            on_selection,
            on_error,
        );
        Ok(())
    } else {
        let _id = id;
        let _content = content;
        let _editable = editable;
        let _on_ready = on_ready;
        let _on_change = on_change;
        let _on_selection = on_selection;
        let _on_error = on_error;
        Ok(())
    }}
}

pub(crate) fn destroy(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::destroy(id);
    } else {
        let _id = id;
    }}
}

pub(crate) fn command(
    id: String,
    generation: u32,
    command: EditorCommand,
) -> Result<(), TiptapEditorError> {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        let _operation = command.operation_name();
        invoke_bridge::<_, serde::de::IgnoredAny>(
            &CommandInvocation {
                id,
                generation,
                command,
            },
            js::command,
        )
        .map(|_| ())
    } else {
        let _id = id;
        let _generation = generation;
        let _command = command;
        Ok(())
    }}
}

pub(crate) fn document(
    id: String,
    generation: u32,
    request: DocumentRequest,
) -> Result<DocumentResponse, TiptapEditorError> {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        let _operation = request.operation_name();
        invoke_bridge(
            &DocumentInvocation {
                id,
                generation,
                request,
            },
            js::document,
        )
    } else {
        let _id = id;
        let _generation = generation;
        let _request = request;
        Err(TiptapEditorError::EditorUnavailable)
    }}
}

#[cfg(all(test, not(feature = "ssr")))]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn maps_command_rejected_errors_with_operation_context() {
        assert_that!(map_bridge_error(JsInteropError {
            kind: "command_rejected".to_owned(),
            message: "selection required".to_owned(),
            operation: Some("toggle_bold".to_owned()),
        }))
        .is_equal_to(TiptapEditorError::CommandRejected {
            operation: "toggle_bold".to_owned(),
            message: "selection required".to_owned(),
        });
    }

    #[test]
    fn reports_missing_success_values_as_bridge_errors() {
        assert_that!(response_to_result::<String>(JsInteropResponse {
            ok: true,
            value: None,
            error: None,
        }))
        .is_err()
        .is_equal_to(TiptapEditorError::BridgeError(
            "received an ok JS bridge response without a value".to_owned(),
        ));
    }
}
