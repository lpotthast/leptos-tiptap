use assertr::prelude::*;
use js_sys::{Array, JSON, Map, Reflect};
use serde_json::json;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

use super::{JsInteropResponse, deserialize_response, response_to_result, serialize_request};
use crate::protocol::{
    CommandInvocation, ContentPayload, DocumentInvocation, DocumentRequest, DocumentResponse,
    EditorCommand, EmptyResponse, ReadyPayload, SetContentOptionsPayload,
    TiptapParseOptionsPayload, TiptapWhitespaceModeKeyword, TiptapWhitespaceModePayload,
};
use crate::{TiptapActiveKey, TiptapAttributes, TiptapEditorError};

fn property(value: &JsValue, name: &str) -> JsValue {
    Reflect::get(value, &JsValue::from_str(name)).expect("property access should succeed")
}

fn parsed_json(value: serde_json::Value) -> JsValue {
    JSON::parse(&value.to_string()).expect("test JSON should parse")
}

fn assert_plain_object(value: &JsValue) {
    assert_that!(value.is_object()).is_true();
    assert_that!(Array::is_array(value)).is_false();
    assert_that!(value.is_instance_of::<Map>()).is_false();
}

fn assert_string_property(value: &JsValue, name: &str, expected: &str) {
    assert_that!(property(value, name).as_string()).is_equal_to(Some(expected.to_owned()));
}

#[wasm_bindgen_test]
fn create_request_matches_typescript_shape_and_uses_plain_json_objects() {
    let request = crate::protocol::CreateRequest {
        id: "json-editor".to_owned(),
        content: ContentPayload::Json(json!({
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "attrs": {"textAlign": "center"},
                "content": [{"type": "text", "text": "Initial JSON"}]
            }]
        })),
        editable: true,
        extensions: vec!["document", "paragraph", "text"],
        placeholder: None,
    };

    let request = serialize_request(&request).expect("create request should serialize");
    assert_plain_object(&request);
    assert_string_property(&request, "id", "json-editor");
    assert_that!(property(&request, "editable").as_bool()).is_equal_to(Some(true));
    assert_that!(property(&request, "placeholder").is_undefined()).is_true();

    let extensions = property(&request, "extensions");
    assert_that!(Array::is_array(&extensions)).is_true();
    let extensions = Array::from(&extensions);
    assert_that!(extensions.length()).is_equal_to(3);
    assert_that!(extensions.get(0).as_string()).is_equal_to(Some("document".to_owned()));

    let content = property(&request, "content");
    assert_plain_object(&content);
    assert_string_property(&content, "format", "json");

    let document = property(&content, "value");
    assert_plain_object(&document);
    assert_string_property(&document, "type", "doc");

    let blocks = property(&document, "content");
    assert_that!(Array::is_array(&blocks)).is_true();
    let paragraph = Array::from(&blocks).get(0);
    assert_plain_object(&paragraph);
    let attributes = property(&paragraph, "attrs");
    assert_plain_object(&attributes);
    assert_string_property(&attributes, "textAlign", "center");
}

#[wasm_bindgen_test]
fn document_request_preserves_nested_option_types_and_json_objects() {
    let request = DocumentInvocation {
        id: "json-editor".to_owned(),
        generation: 7,
        request: DocumentRequest::SetContent {
            content: ContentPayload::Json(json!({
                "type": "doc",
                "content": [{"type": "paragraph"}]
            })),
            options: Some(SetContentOptionsPayload {
                emit_update: Some(false),
                parse_options: Some(TiptapParseOptionsPayload {
                    preserve_whitespace: Some(TiptapWhitespaceModePayload::Full(
                        TiptapWhitespaceModeKeyword::Full,
                    )),
                    from: Some(1),
                    to: None,
                }),
                error_on_invalid_content: Some(true),
            }),
        },
    };

    let invocation = serialize_request(&request).expect("document request should serialize");
    assert_plain_object(&invocation);
    assert_that!(property(&invocation, "generation").as_f64()).is_equal_to(Some(7.0));

    let request = property(&invocation, "request");
    assert_plain_object(&request);
    assert_string_property(&request, "kind", "set_content");

    let content = property(&request, "content");
    assert_plain_object(&content);
    assert_plain_object(&property(&content, "value"));

    let options = property(&request, "options");
    assert_plain_object(&options);
    assert_that!(property(&options, "emit_update").as_bool()).is_equal_to(Some(false));
    assert_that!(property(&options, "error_on_invalid_content").as_bool()).is_equal_to(Some(true));

    let parse_options = property(&options, "parse_options");
    assert_plain_object(&parse_options);
    assert_string_property(&parse_options, "preserve_whitespace", "full");
    assert_that!(property(&parse_options, "from").as_f64()).is_equal_to(Some(1.0));
    assert_that!(property(&parse_options, "to").is_undefined()).is_true();
}

#[wasm_bindgen_test]
fn command_requests_use_plain_objects_for_attributes_and_metadata() {
    let attributes = [
        ("href", json!("https://example.com/bridge")),
        ("data", json!({"source": "wasm", "verified": true})),
    ]
    .into_iter()
    .collect::<TiptapAttributes>();
    let set_mark = CommandInvocation {
        id: "json-editor".to_owned(),
        generation: 7,
        command: EditorCommand::SetMark {
            type_or_name: "link".to_owned(),
            attributes: Some(attributes),
        },
    };

    let invocation = serialize_request(&set_mark).expect("set_mark request should serialize");
    let command = property(&invocation, "command");
    assert_plain_object(&command);
    assert_string_property(&command, "kind", "set_mark");
    let attributes = property(&command, "attributes");
    assert_plain_object(&attributes);
    assert_string_property(&attributes, "href", "https://example.com/bridge");
    let data = property(&attributes, "data");
    assert_plain_object(&data);
    assert_string_property(&data, "source", "wasm");
    assert_that!(property(&data, "verified").as_bool()).is_equal_to(Some(true));

    let set_meta = CommandInvocation {
        id: "json-editor".to_owned(),
        generation: 7,
        command: EditorCommand::SetMeta {
            key: "bridge_abi".to_owned(),
            value: json!({"nested": {"count": 2}}),
        },
    };
    let invocation = serialize_request(&set_meta).expect("set_meta request should serialize");
    let command = property(&invocation, "command");
    assert_string_property(&command, "kind", "set_meta");
    let value = property(&command, "value");
    assert_plain_object(&value);
    let nested = property(&value, "nested");
    assert_plain_object(&nested);
    assert_that!(property(&nested, "count").as_f64()).is_equal_to(Some(2.0));
}

#[wasm_bindgen_test]
fn bridge_success_results_deserialize_ready_and_content_payloads() {
    let ready_response = parsed_json(json!({
        "ok": true,
        "value": {
            "generation": 42,
            "selection_state": {"active": {"bold": true}}
        }
    }));
    let ready: JsInteropResponse<ReadyPayload> =
        deserialize_response(ready_response).expect("ready result should deserialize");
    let ready = response_to_result(ready).expect("ready result should be successful");
    assert_that!(ready.generation).is_equal_to(42);
    assert_that!(ready.selection_state.is_active(TiptapActiveKey::Bold)).is_true();

    let content_response = parsed_json(json!({
        "ok": true,
        "value": {
            "kind": "content",
            "content": {
                "format": "json",
                "value": {"type": "doc", "content": []}
            }
        }
    }));
    let content: JsInteropResponse<DocumentResponse> =
        deserialize_response(content_response).expect("content result should deserialize");
    assert_that!(response_to_result(content).unwrap()).is_equal_to(DocumentResponse::Content {
        content: ContentPayload::Json(json!({"type": "doc", "content": []})),
    });
}

#[wasm_bindgen_test]
fn bridge_error_results_cover_every_typescript_error_kind() {
    let cases = [
        ("editor_unavailable", None, TiptapEditorError::Stale),
        (
            "editor_mount_failed",
            None,
            TiptapEditorError::MountFailed("failure".to_owned()),
        ),
        (
            "duplicate_editor_id",
            None,
            TiptapEditorError::DuplicateEditorId("failure".to_owned()),
        ),
        (
            "invalid_content",
            None,
            TiptapEditorError::InvalidContent("failure".to_owned()),
        ),
        (
            "command_rejected",
            Some("set_mark"),
            TiptapEditorError::CommandRejected {
                operation: "set_mark".to_owned(),
                message: "failure".to_owned(),
            },
        ),
        (
            "operation_failed",
            Some("set_content"),
            TiptapEditorError::OperationFailed {
                operation: "set_content".to_owned(),
                message: "failure".to_owned(),
            },
        ),
        (
            "extension_unavailable",
            Some("set_link"),
            TiptapEditorError::OperationFailed {
                operation: "set_link".to_owned(),
                message: "failure".to_owned(),
            },
        ),
        (
            "extension_registration_failed",
            None,
            TiptapEditorError::BridgeError("failure".to_owned()),
        ),
    ];

    for (kind, operation, expected) in cases {
        let mut error = json!({
            "kind": kind,
            "message": "failure",
        });
        if let Some(operation) = operation {
            error["operation"] = json!(operation);
        }
        let response = parsed_json(json!({
            "ok": false,
            "error": error,
        }));
        let response: JsInteropResponse<EmptyResponse> =
            deserialize_response(response).expect("error result should deserialize");
        assert_that!(response_to_result(response).unwrap_err()).is_equal_to(expected);
    }
}

#[wasm_bindgen_test]
fn bridge_results_reject_wrong_required_field_types_and_missing_payloads() {
    let wrong_generation = parsed_json(json!({
        "ok": true,
        "value": {
            "generation": "42",
            "selection_state": {"active": {}}
        }
    }));
    assert_that!(deserialize_response::<ReadyPayload>(wrong_generation).is_err()).is_true();

    let missing_value = parsed_json(json!({"ok": true}));
    let response: JsInteropResponse<EmptyResponse> =
        deserialize_response(missing_value).expect("outer result should deserialize");
    assert_that!(response_to_result(response)).is_err();

    let missing_error = parsed_json(json!({"ok": false}));
    let response: JsInteropResponse<EmptyResponse> =
        deserialize_response(missing_error).expect("outer result should deserialize");
    assert_that!(response_to_result(response)).is_err();
}
