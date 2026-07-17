use super::{
    TiptapContent, TiptapEditorHandle, TiptapEditorReport, TiptapExtension, TiptapSelectionState,
    use_tiptap_editor::{UseTiptapEditorInput, use_tiptap_editor},
};
use leptos::prelude::*;
use leptos_classes::{Classes, MergeStrategy};
use leptos_styles::Styles;

/// Mounts a Tiptap editor and connects it to a reactive [`TiptapEditorHandle`].
///
/// Create one handle per logical editor, pass it together with a globally unique `id` and the
/// editor's one-time `initial_content`, then use the handle to observe readiness, run commands,
/// and read or replace the document. Operations attempted before the editor is ready return
/// [`TiptapEditorError::NotReady`](super::TiptapEditorError::NotReady).
///
/// To choose and compose the host element yourself, or to build a custom editor component from
/// scratch, use [`use_tiptap_editor`] instead.
///
/// For a small working application, see the
/// [demo app component](https://github.com/lpotthast/leptos-tiptap/blob/main/examples/demo-app/src/lib.rs).
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use leptos_tiptap::{TiptapContent, TiptapEditor, TiptapEditorHandle};
///
/// #[component]
/// fn Editor() -> impl IntoView {
///     let handle = TiptapEditorHandle::new();
///
///     view! {
///         <TiptapEditor
///             id="article-editor"
///             handle=handle
///             initial_content=TiptapContent::html("<p>Edit me.</p>")
///         />
///     }
/// }
/// ```
#[component]
pub fn TiptapEditor(
    /// The ID for this tiptap instance. Must be UNIQUE across ALL instances.
    ///
    /// You might want to use a UUID (v4 or v7) if uniqueness is otherwise not enforceable.
    #[prop(into)]
    id: String,

    /// A reactive editor handle. The component populates this with a live instance when the
    /// editor is ready and records creation failure or cleanup as precise terminal states.
    ///
    /// Use methods on the `TiptapEditorHandle` to send commands or read content.
    /// Use `TiptapEditorHandle::is_ready()` to reactively gate UI.
    ///
    /// Use one handle per logical editor. The same handle may survive sequential conditional
    /// unmount/remount or retry, but must not be shared by distinct or concurrently mounted
    /// editors.
    ///
    /// Commands executed before the editor initialization finishes are answered with a
    /// `TiptapEditorError::NotReady` error.
    handle: TiptapEditorHandle,

    /// Initial content of the editor.
    ///
    /// The editor keeps an internal copy of this content and can solely work with that for an
    /// unlimited time. Changes made to this content by the user, by performing edits, are not
    /// given back immediately. Instead, `on_change` is called to notify you about the change. You
    /// can then decide for yourself whether you want to fetch the updated content immediately,
    /// using your `handle` or if you just want to mark the editor content as dirty to be
    /// fetched later, when needed.
    ///
    /// If you need to replace the visible content later, use `TiptapEditorHandle::set_content`.
    #[prop(into)]
    initial_content: TiptapContent,

    /// Additional classes for the editor host element.
    ///
    /// The `leptos-tiptap-instance` class is always present. Static and reactive caller-provided
    /// classes are composed through [`Classes`].
    #[prop(into, optional)]
    classes: Classes,

    /// Inline styles for the editor host element.
    ///
    /// Static and reactive declarations are rendered through [`Styles`].
    #[prop(into, optional)]
    styles: Styles,

    /// Called once the editor instance exists and has been populated into `handle`.
    ///
    /// This is a convenient one-shot readiness notification for code that does not want to watch
    /// `handle.is_ready()` reactively.
    #[prop(into, optional)]
    on_ready: Option<Callback<()>>,

    /// Called whenever the editor content changes.
    /// Use `handle` to pull the current editor content in whichever format you need.
    #[prop(into, optional)]
    on_change: Option<Callback<()>>,

    /// Called whenever the JS bridge reports a runtime error.
    #[prop(into, optional)]
    on_error: Option<Callback<TiptapEditorReport>>,

    /// If set to true, the tiptap instance becomes un-editable.
    /// The instance reacts to changes of this signal's value.
    #[prop(into, optional)]
    disabled: Signal<bool>,

    /// The set of compiled extensions that should be active for this editor instance.
    ///
    /// If omitted, all extensions enabled through Cargo features are activated.
    /// This is one-time initialization input, just like `initial_content`.
    #[prop(into, optional)]
    extensions: Option<Vec<TiptapExtension>>,

    /// Placeholder text used by the Tiptap placeholder extension during editor initialization.
    ///
    /// This is one-time initialization input and only has an effect when the placeholder extension
    /// is enabled and active for this editor.
    ///
    /// The placeholder extension adds empty-node classes and `data-placeholder` attributes, but
    /// visible placeholder text still requires app CSS, such as rendering
    /// `content: attr(data-placeholder)` in a `::before` pseudo-element.
    ///
    /// See the official Tiptap Placeholder docs for CSS examples:
    /// <https://tiptap.dev/docs/editor/extensions/functionality/placeholder>.
    #[prop(into, optional)]
    placeholder: Option<String>,

    /// Notifies you about a new selection. A selection changes, for example, if the cursor in the
    /// editor changes position, "selecting" a new element in the editor.
    #[prop(into, optional)]
    on_selection_change: Option<Callback<TiptapSelectionState>>,
) -> impl IntoView {
    let hook = use_tiptap_editor(UseTiptapEditorInput {
        id,
        handle: Some(handle),
        initial_content,
        on_ready,
        on_change,
        on_selection_change,
        on_error,
        disabled,
        extensions,
        placeholder,
    });
    let classes = Classes::from("leptos-tiptap-instance").merge(classes, MergeStrategy::KeepSelf);

    view! {
        <div
            class={classes}
            style={styles}
            {..hook.props.into_attrs()}
        ></div>
    }
}
