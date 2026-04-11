#[cfg(not(feature = "ssr"))]
use crate::protocol::ReadyPayload;
use crate::runtime::{self};
#[cfg(not(feature = "ssr"))]
use crate::runtime::{CreateCallbacks, CreateOptions};
use crate::{
    TiptapContent, TiptapEditor, TiptapEditorError, TiptapExtension, TiptapSelectionState,
};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::ScopedClosure;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::prelude::Closure;

/// Stored closures, called by the TipTap JS runtime.
struct EditorCallbacks {
    _on_ready: SendWrapper<ScopedClosure<'static, dyn Fn(JsValue)>>,
    _on_content_change: SendWrapper<ScopedClosure<'static, dyn Fn()>>,
    _on_selection_change: SendWrapper<ScopedClosure<'static, dyn Fn(JsValue)>>,
    _on_error: SendWrapper<ScopedClosure<'static, dyn Fn(JsValue)>>,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditorLifecycle {
    Idle,
    Creating,
    Ready { generation: u32 },
}

#[derive(Clone, Copy)]
pub(crate) struct EditorSession {
    editor_id: StoredValue<String>,
    lifecycle: StoredValue<EditorLifecycle>,
    callbacks: StoredValue<Option<EditorCallbacks>>,
    applied_editable: StoredValue<Option<bool>>,
    editor: TiptapEditor,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
pub(crate) struct EditorSessionMountOptions {
    pub(crate) initial_content: TiptapContent,
    pub(crate) initial_editable: bool,
    pub(crate) extensions: Vec<TiptapExtension>,
    pub(crate) on_ready: Option<Callback<()>>,
    pub(crate) on_change: Option<Callback<()>>,
    pub(crate) on_error: Option<Callback<TiptapEditorError>>,
    pub(crate) on_selection_change: Option<Callback<TiptapSelectionState>>,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
fn reset_local_editor_state(
    lifecycle: StoredValue<EditorLifecycle>,
    callbacks: StoredValue<Option<EditorCallbacks>>,
    applied_editable: StoredValue<Option<bool>>,
    editor: TiptapEditor,
) {
    lifecycle.update_value(|state| *state = EditorLifecycle::Idle);
    callbacks.update_value(|slot| *slot = None);
    applied_editable.update_value(|value| *value = None);
    editor.clear_handle();
}

fn report_runtime_error(on_error: Option<Callback<TiptapEditorError>>, err: TiptapEditorError) {
    tracing::error!(?err, "TipTap runtime error.");
    on_error.inspect(move |cb| cb.run(err));
}

impl EditorSession {
    pub(crate) fn new(id: String, editor: TiptapEditor) -> Self {
        Self {
            editor_id: StoredValue::new(id),
            lifecycle: StoredValue::new(EditorLifecycle::Idle),
            callbacks: StoredValue::new(Option::<EditorCallbacks>::None),
            applied_editable: StoredValue::new(Option::<bool>::None),
            editor,
        }
    }

    pub(crate) fn id(self) -> String {
        self.editor_id.get_value()
    }

    pub(crate) fn is_idle(self) -> bool {
        self.lifecycle.read_value() == EditorLifecycle::Idle
    }

    pub(crate) fn mount(self, options: EditorSessionMountOptions) {
        #[cfg(not(feature = "ssr"))]
        {
            let EditorSessionMountOptions {
                initial_content,
                initial_editable,
                extensions,
                on_ready,
                on_change,
                on_error,
                on_selection_change,
            } = options;

            let initial_content = match crate::protocol::ContentPayload::try_from(initial_content) {
                Ok(initial_content) => initial_content,
                Err(err) => {
                    report_runtime_error(on_error, err);
                    return;
                }
            };

            if let Err(err) = TiptapExtension::validate_extension_set(&extensions) {
                report_runtime_error(on_error, err);
                return;
            }

            let editor_id = self.editor_id;
            let lifecycle = self.lifecycle;
            let callbacks = self.callbacks;
            let applied_editable = self.applied_editable;
            let editor = self.editor;

            let on_error_for_ready = on_error.clone();
            let on_ready_closure = SendWrapper::new(Closure::new(move |ready_as_js_value| {
                let ready: ReadyPayload = match serde_wasm_bindgen::from_value(ready_as_js_value) {
                    Ok(ready) => ready,
                    Err(err) => {
                        runtime::destroy(editor_id.get_value());
                        reset_local_editor_state(lifecycle, callbacks, applied_editable, editor);
                        report_runtime_error(
                            on_error_for_ready,
                            TiptapEditorError::BridgeError(format!(
                                "could not parse ready payload from JS: {err}"
                            )),
                        );
                        return;
                    }
                };

                lifecycle.update_value(|state| {
                    *state = EditorLifecycle::Ready {
                        generation: ready.generation,
                    };
                });
                editor.set_handle(crate::TiptapEditorHandle::new(
                    editor_id.get_value(),
                    ready.generation,
                ));
                on_ready.inspect(|cb| cb.run(()));
            }));

            let on_content_change_closure = SendWrapper::new(Closure::new(move || {
                if matches!(*lifecycle.read_value(), EditorLifecycle::Ready { .. }) {
                    on_change.inspect(|cb| cb.run(()));
                }
            }));

            let on_error_for_selection = on_error.clone();
            let on_selection_change_closure =
                SendWrapper::new(Closure::new(move |selection_state_as_js_value| {
                    let selection_state: TiptapSelectionState =
                        match serde_wasm_bindgen::from_value(selection_state_as_js_value) {
                            Ok(selection_state) => selection_state,
                            Err(err) => {
                                report_runtime_error(
                                    on_error_for_selection,
                                    TiptapEditorError::InvalidJson(format!(
                                        "could not parse selection state from JS: {err}"
                                    )),
                                );
                                return;
                            }
                        };

                    on_selection_change.inspect(|cb| cb.run(selection_state));
                }));

            let on_error_closure = SendWrapper::new(Closure::new(move |error_as_js_value| {
                let err = runtime::error_from_js_value(error_as_js_value);
                reset_local_editor_state(lifecycle, callbacks, applied_editable, editor);
                report_runtime_error(on_error, err);
            }));

            lifecycle.update_value(|state| *state = EditorLifecycle::Creating);
            applied_editable.update_value(|value| *value = Some(initial_editable));
            callbacks.update_value(|slot| {
                *slot = Some(EditorCallbacks {
                    _on_ready: on_ready_closure,
                    _on_content_change: on_content_change_closure,
                    _on_selection_change: on_selection_change_closure,
                    _on_error: on_error_closure,
                });
            });

            let stored_callbacks = callbacks.read_value();
            let Some(editor_callbacks) = stored_callbacks.as_ref() else {
                reset_local_editor_state(lifecycle, callbacks, applied_editable, editor);
                return;
            };

            if let Err(err) = runtime::create(
                CreateOptions {
                    id: editor_id.get_value(),
                    content: initial_content,
                    editable: initial_editable,
                    extensions,
                },
                CreateCallbacks {
                    on_ready: &editor_callbacks._on_ready,
                    on_change: &editor_callbacks._on_content_change,
                    on_selection: &editor_callbacks._on_selection_change,
                    on_error: &editor_callbacks._on_error,
                },
            ) {
                reset_local_editor_state(lifecycle, callbacks, applied_editable, editor);
                report_runtime_error(on_error, err);
            }
        }

        #[cfg(feature = "ssr")]
        {
            let _ = options;
        }
    }

    pub(crate) fn sync_editable(
        self,
        desired_editable: bool,
        on_error: Option<Callback<TiptapEditorError>>,
    ) {
        if self.editor.is_ready()
            && let EditorLifecycle::Ready { generation } = *self.lifecycle.read_value()
            && self.applied_editable.read_value() != Some(desired_editable)
        {
            if let Err(err) = runtime::command(
                self.editor_id.get_value(),
                generation,
                crate::protocol::EditorCommand::SetEditable {
                    editable: desired_editable,
                },
            ) {
                report_runtime_error(on_error, err);
            } else {
                self.applied_editable
                    .update_value(|value| *value = Some(desired_editable));
            }
        }
    }

    pub(crate) fn cleanup(self) {
        if self.lifecycle.read_value() != EditorLifecycle::Idle {
            runtime::destroy(self.editor_id.get_value());
        }

        reset_local_editor_state(
            self.lifecycle,
            self.callbacks,
            self.applied_editable,
            self.editor,
        );
    }
}
