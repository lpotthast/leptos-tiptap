use super::{
    TiptapContent, TiptapEditorError, TiptapEditorHandle, TiptapExtension, TiptapSelectionState,
};
use crate::runtime::{TiptapRuntimeMountOptions, TiptapRuntimeSession};
use leptos::{attr, attr::Attr, prelude::*};
use leptos_element_capture::{CapturedElement, ElementCaptureAttr};

/// Input parameters for the `use_tiptap_editor` hook.
#[derive(Clone)]
pub struct UseTiptapEditorInput {
    /// The ID for this Tiptap instance. Must be unique across all mounted instances.
    pub id: String,

    /// An optional editor handle to populate when the editor becomes ready.
    ///
    /// If omitted, the hook creates and returns its own [`struct@TiptapEditorHandle`].
    pub editor: Option<TiptapEditorHandle>,

    /// Initial content of the editor.
    pub initial_content: TiptapContent,

    /// Called once the editor has been populated into `editor`.
    pub on_ready: Option<Callback<()>>,

    /// Called whenever the editor content changes.
    pub on_change: Option<Callback<()>>,

    /// Called whenever the current editor selection changes.
    pub on_selection_change: Option<Callback<TiptapSelectionState>>,

    /// Called whenever the JS bridge reports a runtime error.
    pub on_error: Option<Callback<TiptapEditorError>>,

    /// Whether editing should be disabled.
    pub disabled: Signal<bool>,

    /// The set of compiled extensions that should be active for this editor instance.
    ///
    /// If omitted, all extensions enabled through Cargo features are activated.
    pub extensions: Option<Vec<TiptapExtension>>,

    /// Placeholder text used by the Tiptap placeholder extension during editor initialization.
    ///
    /// The placeholder extension adds empty-node classes and `data-placeholder` attributes, but
    /// visible placeholder text still requires app CSS, such as rendering
    /// `content: attr(data-placeholder)` in a `::before` pseudo-element.
    ///
    /// See the official Tiptap Placeholder docs for CSS examples:
    /// <https://tiptap.dev/docs/editor/extensions/functionality/placeholder>.
    pub placeholder: Option<String>,
}

/// Mount props returned by [`use_tiptap_editor`].
pub struct UseTiptapEditorProps {
    /// The stable DOM id for this editor instance.
    pub id: String,

    /// Whether the mounted host node should expose `aria-disabled=true`.
    pub aria_disabled: Signal<bool>,

    /// Captures the rendered host element when these props are spread.
    pub element_capture: ElementCaptureAttr,
}

impl UseTiptapEditorProps {
    /// Converts the props into attributes that can be spread onto the host node.
    #[must_use]
    pub fn into_attrs(self) -> UseTiptapEditorAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.element_capture,
        )
    }
}

/// Attribute tuple returned by [`UseTiptapEditorProps::into_attrs`].
pub type UseTiptapEditorAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaDisabled, Signal<bool>>,
    ElementCaptureAttr,
);

/// The return value of the `use_tiptap_editor` hook.
pub struct UseTiptapEditorReturn {
    /// Mount props for the host DOM node.
    pub props: UseTiptapEditorProps,

    /// The reactive editor handle for issuing commands and reading content.
    pub editor: TiptapEditorHandle,

    /// Reactive readiness state for the editor handle.
    pub is_ready: Signal<bool>,

    /// The captured host element for the mounted editor container.
    pub element: CapturedElement,
}

/// Creates and manages a Tiptap editor instance from within a Leptos owner scope.
///
/// This hook owns the Leptos-specific orchestration around `TiptapRuntimeSession`:
/// mount timing, disabled synchronization, cleanup, and exposing the editor handle.
pub fn use_tiptap_editor(input: UseTiptapEditorInput) -> UseTiptapEditorReturn {
    let UseTiptapEditorInput {
        id,
        editor,
        initial_content,
        on_ready,
        on_change,
        on_error,
        disabled,
        extensions,
        placeholder,
        on_selection_change,
    } = input;

    let editor = editor.unwrap_or_default();
    let session = TiptapRuntimeSession::new(id, editor);
    let mount_options = TiptapRuntimeMountOptions {
        initial_content,
        initial_editable: !disabled.get_untracked(),
        extensions: extensions.unwrap_or_else(TiptapExtension::all_enabled),
        placeholder,
        on_ready,
        on_change,
        on_error,
        on_selection_change,
    };

    let element = CapturedElement::new();
    Effect::new({
        let mut mount_options = Some(mount_options);
        move |_| {
            if element.get().is_none() {
                return;
            }

            if !session.is_idle() {
                tracing::warn!("Unexpected TipTap editor reinitialization detected.");
                return;
            }

            let Some(mount_options) = mount_options.take() else {
                tracing::warn!(
                    "Ignored duplicate TipTap editor mount request after mount options were consumed."
                );
                return;
            };

            session.mount(mount_options);
        }
    });

    Effect::new(move |_| {
        session.sync_editable(!disabled.get(), on_error);
    });

    on_cleanup(move || session.cleanup());

    UseTiptapEditorReturn {
        props: UseTiptapEditorProps {
            id: session.id(),
            aria_disabled: disabled,
            element_capture: element.attr(),
        },
        editor,
        is_ready: Signal::derive(move || editor.is_ready()),
        element,
    }
}
