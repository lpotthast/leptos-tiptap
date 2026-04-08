use crate::{
    js_tiptap, TiptapContent, TiptapEditorError, TiptapEditorHandle, TiptapSelectionState,
};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::closure::ScopedClosure;
use wasm_bindgen::{prelude::Closure, JsValue};

/// Stored closures, called by the TipTap JS runtime.
struct EditorCallbacks {
    _on_ready: SendWrapper<ScopedClosure<'static, dyn Fn(JsValue)>>,
    _on_content_change: SendWrapper<ScopedClosure<'static, dyn Fn()>>,
    _on_selection_change: SendWrapper<ScopedClosure<'static, dyn Fn(JsValue)>>,
    _on_error: SendWrapper<ScopedClosure<'static, dyn Fn(JsValue)>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditorLifecycle {
    Idle,
    Creating,
    Ready { generation: u32 },
}

fn sync_disabled_state(id: &str, generation: u32, disabled: bool) -> Result<(), TiptapEditorError> {
    js_tiptap::command(
        id.to_owned(),
        generation,
        js_tiptap::EditorCommand::SetEditable {
            editable: !disabled,
        },
    )
}

fn report_runtime_error(on_error: Option<Callback<TiptapEditorError>>, err: TiptapEditorError) {
    tracing::error!(?err, "TipTap runtime error.");
    on_error.inspect(move |cb| cb.run(err));
}

#[component]
pub fn TiptapInstance(
    /// The ID for this tiptap instance. Must be UNIQUE across ALL instances.
    ///
    /// You might want to use a UUID (v4 or v7) if uniqueness is otherwise not enforceable.
    #[prop(into)]
    id: String,

    /// Initial content of the editor.
    ///
    /// The editor keeps an internal copy of this content and can solely work with that for an
    /// unlimited time. Changes made to this content by the user, by performing edits, are not
    /// given back immediately. Instead, `on_change` is called to notify you about the change. You
    /// can then decide for yourself whether you want to fetch the updated content or not, using
    /// the handle provided in the `on_change` callback. If you need to replace the visible content
    /// later, use `TiptapEditorHandle::set_content`.
    #[prop(into)]
    initial_content: TiptapContent,

    /// Called once the editor instance exists and can be queried through the provided handle.
    #[prop(optional, into)]
    on_ready: Option<Callback<TiptapEditorHandle>>,

    /// Called whenever the editor content changes.
    /// Use the provided handle to pull the current editor content in whichever format you need.
    #[prop(optional, into)]
    on_change: Option<Callback<TiptapEditorHandle>>,

    /// Called whenever the JS bridge reports a runtime error.
    #[prop(optional, into)]
    on_error: Option<Callback<TiptapEditorError>>,

    /// If set to true, the tiptap instance becomes un-editable.
    /// The instance reacts to changes of this signal's value.
    #[prop(into)]
    disabled: Signal<bool>,

    /// Notifies you about a new selection. A selection changes, for example, if the cursor in the
    /// editor changes position, "selecting" a new element in the editor.
    #[prop(optional, into)]
    on_selection_change: Option<Callback<TiptapSelectionState>>,
) -> impl IntoView {
    let instance = NodeRef::new();
    let callbacks = StoredValue::new(Option::<EditorCallbacks>::None);
    let editor_id = StoredValue::new(id);
    let lifecycle = StoredValue::new(EditorLifecycle::Idle);

    Effect::new(move |_| {
        // Wait for the node to be rendered.
        if instance.get().is_none() {
            return;
        }

        // Avoid accidental reinitialization.
        if lifecycle.read_value() != EditorLifecycle::Idle {
            tracing::warn!("Unexpected TipTap editor reinitialization detected.");
            return;
        }

        let on_ready_closure = SendWrapper::new(Closure::new(move |ready_as_js_value| {
            let ready: js_tiptap::ReadyPayload =
                match serde_wasm_bindgen::from_value(ready_as_js_value) {
                    Ok(ready) => ready,
                    Err(err) => {
                        lifecycle.update_value(|state| *state = EditorLifecycle::Idle);
                        callbacks.update_value(|slot| *slot = None);
                        report_runtime_error(
                            on_error,
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
            on_ready.inspect(|cb| {
                cb.run(TiptapEditorHandle::new(
                    editor_id.get_value(),
                    ready.generation,
                ))
            });
        }));

        let on_content_change_closure = SendWrapper::new(Closure::new(move || {
            let generation = match *lifecycle.read_value() {
                EditorLifecycle::Ready { generation } => generation,
                EditorLifecycle::Idle | EditorLifecycle::Creating => return,
            };

            on_change
                .inspect(|cb| cb.run(TiptapEditorHandle::new(editor_id.get_value(), generation)));
        }));

        let on_selection_change_closure =
            SendWrapper::new(Closure::new(move |selection_state_as_js_value| {
                let selection_state: TiptapSelectionState =
                    match serde_wasm_bindgen::from_value(selection_state_as_js_value) {
                        Ok(selection_state) => selection_state,
                        Err(err) => {
                            report_runtime_error(
                                on_error,
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
            let err = js_tiptap::error_from_js_value(error_as_js_value);
            lifecycle.update_value(|state| *state = EditorLifecycle::Idle);
            callbacks.update_value(|slot| *slot = None);
            report_runtime_error(on_error, err);
        }));

        lifecycle.update_value(|state| *state = EditorLifecycle::Creating);
        if let Err(err) = js_tiptap::create(
            editor_id.get_value(),
            initial_content.clone().into_payload(),
            !disabled.get_untracked(),
            &on_ready_closure,
            &on_content_change_closure,
            &on_selection_change_closure,
            &on_error_closure,
        ) {
            lifecycle.update_value(|state| *state = EditorLifecycle::Idle);
            callbacks.update_value(|slot| *slot = None);
            report_runtime_error(on_error, err);
            return;
        }

        if lifecycle.read_value() != EditorLifecycle::Idle {
            callbacks.update_value(|slot| {
                *slot = Some(EditorCallbacks {
                    _on_ready: on_ready_closure,
                    _on_content_change: on_content_change_closure,
                    _on_selection_change: on_selection_change_closure,
                    _on_error: on_error_closure,
                });
            });
        }
    });

    on_cleanup(move || {
        if lifecycle.read_value() != EditorLifecycle::Idle {
            js_tiptap::destroy(editor_id.get_value());
        }
        let _ = callbacks.write_value().take();
        lifecycle.update_value(|state| *state = EditorLifecycle::Idle);
    });

    // Synchronize the requested disabled state.
    // This re-runs both when `disabled` changes and when `is_ready` changes,
    // so the initial disabled state is synced automatically when the editor becomes ready.
    Effect::new(move |_| {
        let disabled = disabled.get();
        if let EditorLifecycle::Ready { generation } = *lifecycle.read_value() {
            if let Err(err) = sync_disabled_state(&editor_id.get_value(), generation, disabled) {
                report_runtime_error(on_error, err);
            }
        }
    });

    view! {
        <leptos-tiptap-instance
            node_ref=instance
            id=editor_id.get_value()
            aria-disabled=disabled
        />
    }
}
