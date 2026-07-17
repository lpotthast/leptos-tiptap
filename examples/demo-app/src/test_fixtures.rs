//! Browser-test fixtures.
//!
//! Each component below isolates one editor scenario the integration-tests assert:
//!
//! - multiple editors on a page
//! - duplicate live editor IDs
//! - the placeholder extension
//! - a non-default extension subset
//! - the `on_error` callback
//! - the `on_change` callback firing exactly once per programmatic content replacement.
//! - a persistent handle reused after destroy or create failure.
//!
//! The main `DemoApp` deliberately stays unrelated to any test.

use leptos::prelude::*;
use leptos_tiptap::{
    TiptapContent, TiptapEditor, TiptapEditorError, TiptapEditorHandle, TiptapEditorReport,
    TiptapExtension, TiptapHeadingLevel, UseTiptapEditorInput, use_tiptap_editor,
};

#[component]
pub fn MultiEditorFixture() -> impl IntoView {
    let handle_a = TiptapEditorHandle::new();
    let handle_b = TiptapEditorHandle::new();

    view! {
        <section id="multi-editor">
            <h2>"Multi editor"</h2>

            <button
                disabled=move || !handle_a.is_ready()
                on:click=move |_| { let _ = handle_a.toggle_heading(TiptapHeadingLevel::H1); }
            >
                "H1 A"
            </button>
            <button
                disabled=move || !handle_b.is_ready()
                on:click=move |_| { let _ = handle_b.toggle_heading(TiptapHeadingLevel::H2); }
            >
                "H2 B"
            </button>

            <TiptapEditor
                handle=handle_a
                id="editor-a"
                initial_content=TiptapContent::html("<p>Editor A.</p>")
                classes="editor-a"
            />
            <TiptapEditor
                handle=handle_b
                id="editor-b"
                initial_content=TiptapContent::html("<p>Editor B.</p>")
                classes="editor-b"
            />
        </section>
    }
}

#[component]
pub fn DuplicateEditorIdFixture() -> impl IntoView {
    let original_handle = TiptapEditorHandle::new();
    let rejected_handle = TiptapEditorHandle::new();
    let (mount_duplicate, set_mount_duplicate) = signal(false);
    let (original_ready, set_original_ready) = signal(false);
    let (duplicate_error, set_duplicate_error) = signal(String::new());
    let (rejected_state, set_rejected_state) = signal(String::new());

    view! {
        <section id="duplicate-editor-id">
            <h2>"Duplicate editor ID"</h2>

            <button
                disabled=move || !original_handle.is_ready()
                on:click=move |_| {
                    let _ = original_handle.toggle_heading(TiptapHeadingLevel::H1);
                }
            >
                "H1 original"
            </button>
            <button
                disabled=move || !original_ready.get()
                on:click=move |_| set_mount_duplicate.set(true)
            >
                "Mount duplicate editor"
            </button>

            <pre id="duplicate-original-ready">{move || original_ready.get().to_string()}</pre>
            <pre id="duplicate-rejected-state">{move || rejected_state.get()}</pre>
            <pre id="duplicate-error">{move || duplicate_error.get()}</pre>

            <div id="duplicate-original">
                <TiptapEditor
                    handle=original_handle
                    id="duplicate-live-editor"
                    initial_content=TiptapContent::html("<p>Original editor.</p>")
                    on_ready=move |_| set_original_ready.set(true)
                />
            </div>

            <div id="duplicate-rejected">
                {move || {
                    mount_duplicate.get().then(|| {
                        view! {
                            <TiptapEditor
                                handle=rejected_handle
                                id="duplicate-live-editor"
                                initial_content=TiptapContent::html("<p>Rejected editor.</p>")
                                on_error=move |report| {
                                    set_rejected_state
                                        .set(handle_state(rejected_handle).to_owned());
                                    set_duplicate_error.set(format!("{report}"));
                                }
                            />
                        }
                    })
                }}
            </div>
        </section>
    }
}

#[component]
pub fn PlaceholderFixture() -> impl IntoView {
    let handle = TiptapEditorHandle::new();

    view! {
        <section id="placeholder">
            <h2>"Placeholder"</h2>

            <button
                disabled=move || !handle.is_ready()
                on:click=move |_| { let _ = handle.clear_content(true); }
            >
                "Clear"
            </button>

            <TiptapEditor
                handle=handle
                id="placeholder-editor"
                initial_content=TiptapContent::html("")
                placeholder="Type something here..."
                classes="placeholder-editor"
            />
        </section>
    }
}

#[component]
pub fn ExtensionSubsetFixture() -> impl IntoView {
    let handle = TiptapEditorHandle::new();
    let (subset_error, set_subset_error) = signal(String::new());

    view! {
        <section id="extension-subset">
            <h2>"Extension subset"</h2>

            <button
                disabled=move || !handle.is_ready()
                on:click=move |_| { let _ = handle.toggle_heading(TiptapHeadingLevel::H3); }
            >
                "H3"
            </button>
            <button
                disabled=move || !handle.is_ready()
                on:click=move |_| {
                    match handle.toggle_bold() {
                        Ok(()) => set_subset_error.set(String::from("ok")),
                        Err(report) => set_subset_error.set(format!("{report}")),
                    }
                }
            >
                "Bold"
            </button>

            <TiptapEditor
                handle=handle
                id="subset-editor"
                initial_content=TiptapContent::html("<p>Subset.</p>")
                extensions=vec![
                    TiptapExtension::Document,
                    TiptapExtension::Paragraph,
                    TiptapExtension::Text,
                    TiptapExtension::Heading,
                ]
                classes="subset-editor"
            />

            <pre id="subset-error">{ move || subset_error.get() }</pre>
        </section>
    }
}

#[component]
pub fn OnErrorFixture() -> impl IntoView {
    let handle = TiptapEditorHandle::new();
    let (error, set_error) = signal(String::new());

    // `TextAlign` requires `Heading` and `Paragraph` to be present. Mounting with this subset
    // fails synchronous extension validation before the JS bridge is even invoked, which routes
    // a `TiptapEditorReport` through the `on_error` callback we wired up below.
    let invalid_extensions = vec![TiptapExtension::Text, TiptapExtension::TextAlign];

    view! {
        <section id="on-error">
            <h2>"On error"</h2>

            <TiptapEditor
                handle=handle
                id="error-editor"
                initial_content=TiptapContent::html("<p>Hello.</p>")
                extensions=invalid_extensions
                on_error=move |report| set_error.set(format!("{report}"))
            />

            <pre id="error-message">{ move || error.get() }</pre>
        </section>
    }
}

#[component]
pub fn OnChangeCountingFixture() -> impl IntoView {
    let handle = TiptapEditorHandle::new();
    let (count, set_count) = signal(0_u32);

    view! {
        <section id="on-change-counting">
            <h2>"On change counting"</h2>

            <button
                disabled=move || !handle.is_ready()
                on:click=move |_| {
                    let _ = handle.set_content(TiptapContent::html(
                        "<p>Replaced once.</p>",
                    ));
                }
            >
                "Replace"
            </button>

            <TiptapEditor
                handle=handle
                id="on-change-counting-editor"
                initial_content=TiptapContent::html("<p>Initial.</p>")
                on_change=move |_| set_count.update(|n| *n += 1)
            />

            <pre id="on-change-count">{ move || count.get().to_string() }</pre>
        </section>
    }
}

fn handle_state(handle: TiptapEditorHandle) -> &'static str {
    match handle.get_html() {
        Ok(_) => "Ready",
        Err(report) => match report.into_current_context() {
            TiptapEditorError::NotReady => "NotReady",
            TiptapEditorError::Destroyed => "Destroyed",
            TiptapEditorError::CreateFailed => "CreateFailed",
            _ => "OtherError",
        },
    }
}

/// Uses the public hook so the fixture can snapshot the handle immediately after the runtime
/// session claims it, before the host element is captured and the synchronous JS create runs.
#[component]
fn LifecycleEditorFixture(
    #[prop(into)] id: String,
    handle: TiptapEditorHandle,
    initial_content: TiptapContent,
    #[prop(into)] claim_state_id: String,
    #[prop(into)] ready_state_id: String,
    #[prop(into, optional)] on_error: Option<Callback<TiptapEditorReport>>,
) -> impl IntoView {
    let hook = use_tiptap_editor(UseTiptapEditorInput {
        id,
        handle: Some(handle),
        initial_content,
        on_error,
        ..UseTiptapEditorInput::default()
    });
    let claimed_state = handle_state(handle);
    let is_ready = hook.is_ready;
    let attrs = hook.props.into_attrs();

    view! {
        <pre id=claim_state_id>{claimed_state}</pre>
        <pre id=ready_state_id>{move || is_ready.get().to_string()}</pre>
        <div {..attrs}></div>
    }
}

#[component]
pub fn RemountHandleFixture() -> impl IntoView {
    let handle = TiptapEditorHandle::new();
    let (mounted, set_mounted) = signal(true);
    let (inspected_state, set_inspected_state) = signal(String::new());

    view! {
        <section id="remount-handle">
            <h2>"Remount handle"</h2>

            <button on:click=move |_| set_mounted.set(false)>"Unmount editor"</button>
            <button on:click=move |_| set_mounted.set(true)>"Remount editor"</button>
            <button on:click=move |_| set_inspected_state.set(handle_state(handle).to_owned())>
                "Inspect remount handle"
            </button>

            <pre id="remount-inspected-state">{move || inspected_state.get()}</pre>

            {move || {
                mounted.get().then(|| {
                    view! {
                        <LifecycleEditorFixture
                            id="remount-editor"
                            handle=handle
                            initial_content=TiptapContent::html("<p>Remounted editor.</p>")
                            claim_state_id="remount-claim-state"
                            ready_state_id="remount-ready-state"
                        />
                    }
                })
            }}
        </section>
    }
}

#[component]
pub fn RetryHandleFixture() -> impl IntoView {
    let handle = TiptapEditorHandle::new();
    let (retry, set_retry) = signal(false);
    let (failed_state, set_failed_state) = signal(String::new());
    let (inspected_state, set_inspected_state) = signal(String::new());

    view! {
        <section id="retry-handle">
            <h2>"Retry handle"</h2>

            <button on:click=move |_| set_retry.set(true)>"Retry editor"</button>
            <button on:click=move |_| set_inspected_state.set(handle_state(handle).to_owned())>
                "Inspect retry handle"
            </button>

            <pre id="retry-failed-state">{move || failed_state.get()}</pre>
            <pre id="retry-inspected-state">{move || inspected_state.get()}</pre>

            {move || {
                if retry.get() {
                    Some(
                        view! {
                            <LifecycleEditorFixture
                                id="retry-editor"
                                handle=handle
                                initial_content=TiptapContent::html("<p>Retried editor.</p>")
                                claim_state_id="retry-claim-state"
                                ready_state_id="retry-ready-state"
                            />
                        }
                            .into_any(),
                    )
                } else {
                    Some(
                        view! {
                            <TiptapEditor
                                handle=handle
                                id="retry-editor"
                                initial_content=TiptapContent::html("<p>Failed editor.</p>")
                                extensions=vec![
                                    TiptapExtension::Text,
                                    TiptapExtension::TextAlign,
                                ]
                                on_error=move |_| {
                                    set_failed_state.set(handle_state(handle).to_owned());
                                }
                            />
                        }
                            .into_any(),
                    )
                }
            }}
        </section>
    }
}
