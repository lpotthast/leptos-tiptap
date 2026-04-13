#[cfg(not(feature = "ssr"))]
use crate::protocol::EmptyResponse;
#[cfg(not(feature = "ssr"))]
use crate::protocol::{CommandInvocation, CreateRequest, DocumentInvocation};
use crate::protocol::{ContentPayload, DocumentRequest, DocumentResponse, EditorCommand};
use crate::{TiptapEditorError, TiptapExtension};
use cfg_if::cfg_if;
#[cfg(not(feature = "ssr"))]
use serde::Deserialize;
#[cfg(not(feature = "ssr"))]
use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::ScopedClosure;

#[cfg(not(feature = "ssr"))]
use super::{ffi, registration};

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
        "duplicate_editor_id" => TiptapEditorError::DuplicateEditorId(message),
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
fn format_js_value(value: &JsValue) -> String {
    value.as_string().unwrap_or_else(|| format!("{value:?}"))
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
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

#[cfg_attr(feature = "ssr", allow(dead_code))]
#[derive(Clone, Copy)]
pub(crate) struct CreateCallbacks<'a> {
    pub(crate) ready: &'a ScopedClosure<'static, dyn Fn(JsValue)>,
    pub(crate) change: &'a ScopedClosure<'static, dyn Fn()>,
    pub(crate) selection: &'a ScopedClosure<'static, dyn Fn(JsValue)>,
    pub(crate) error: &'a ScopedClosure<'static, dyn Fn(JsValue)>,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
pub(crate) struct CreateOptions {
    pub(crate) id: String,
    pub(crate) content: ContentPayload,
    pub(crate) editable: bool,
    pub(crate) extensions: Vec<TiptapExtension>,
    pub(crate) placeholder: Option<String>,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
#[cfg_attr(feature = "ssr", allow(clippy::unnecessary_wraps))]
pub(crate) fn create(
    request: CreateOptions,
    callbacks: CreateCallbacks<'_>,
) -> Result<(), TiptapEditorError> {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        registration::ensure_compiled_extensions_registered()?;

        let request = serialize_request(&CreateRequest {
            id: request.id,
            content: request.content,
            editable: request.editable,
            extensions: request.extensions
                .into_iter()
                .map(TiptapExtension::js_name)
                .collect(),
            placeholder: request.placeholder,
        })?;

        ffi::create(
            request,
            callbacks.ready,
            callbacks.change,
            callbacks.selection,
            callbacks.error,
        )
        .map_err(|value| {
            TiptapEditorError::BridgeError(format!(
                "JS bridge create threw an exception: {}",
                format_js_value(&value),
            ))
        })?;
        Ok(())
    } else {
        drop((request, callbacks));
        Ok(())
    }}
}

pub(crate) fn destroy(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        ffi::destroy(id);
    } else {
        drop(id);
    }}
}

#[cfg_attr(feature = "ssr", allow(clippy::unnecessary_wraps))]
pub(crate) fn command(
    id: String,
    generation: u32,
    command: EditorCommand,
) -> Result<(), TiptapEditorError> {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        let _operation = command.operation_name();
        invoke_bridge::<_, EmptyResponse>(
            &CommandInvocation {
                id,
                generation,
                command,
            },
            ffi::command,
        )
        .map(|_| ())
    } else {
        drop((id, generation, command));
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
            ffi::document,
        )
    } else {
        drop((id, generation, request));
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

    #[test]
    fn accepts_explicit_empty_success_values() {
        assert_that!(response_to_result::<EmptyResponse>(JsInteropResponse {
            ok: true,
            value: Some(EmptyResponse::Empty),
            error: None,
        }))
        .is_ok()
        .is_equal_to(EmptyResponse::Empty);
    }
}
