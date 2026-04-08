use crate::TiptapEditorError;
use cfg_if::cfg_if;
use serde::Deserialize;
#[cfg(not(feature = "ssr"))]
use serde::{de::DeserializeOwned, Serialize};
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CreateRequest {
    id: String,
    content: ContentPayload,
    editable: bool,
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

    #[wasm_bindgen(raw_module = "/js/tiptap.js")]
    extern "C" {
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
) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        let request = serialize_request(&CreateRequest {
            id,
            content,
            editable,
        })
        .expect("serializing the Tiptap create request should not fail");

        js::create(
            request,
            on_ready,
            on_change,
            on_selection,
            on_error,
        );
    } else {
        let _id = id;
        let _content = content;
        let _editable = editable;
        let _on_ready = on_ready;
        let _on_change = on_change;
        let _on_selection = on_selection;
        let _on_error = on_error;
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
