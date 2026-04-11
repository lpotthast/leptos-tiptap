use super::TiptapEditorError;
use leptos::prelude::*;

/// A reactive editor slot that the `<TiptapEditor/>` component populates with a live
/// [`TiptapEditorHandle`] when the editor is ready, and clears on error or cleanup.
///
/// Create one with [`TiptapEditor::new()`] and pass it to `<TiptapEditor/>` via the `editor`
/// prop. All command and content methods delegate to the inner handle, returning
/// [`TiptapEditorError::EditorUnavailable`] when the editor is not yet ready.
///
/// `TiptapEditor` is [`Copy`], so it can be freely captured in closures without cloning.
#[derive(Clone, Copy)]
pub struct TiptapEditor(RwSignal<Option<TiptapEditorHandle>>);

impl Default for TiptapEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TiptapEditor {
    /// Creates a new, empty editor slot.
    ///
    /// Must be called within a reactive owner scope (e.g. inside a component body).
    pub fn new() -> Self {
        Self(RwSignal::new(None))
    }

    /// Returns `true` when the underlying editor is ready.
    ///
    /// This is a reactive read; calling it inside an `Effect` or a derived signal
    /// will re-run when readiness changes.
    pub fn is_ready(&self) -> bool {
        self.0.with(|h| h.is_some())
    }

    /// Returns the current handle, if the editor is ready.
    ///
    /// This is a reactive read; calling it inside an `Effect` or a derived signal
    /// will re-run when readiness changes.
    pub fn handle(&self) -> Option<TiptapEditorHandle> {
        self.0.get()
    }

    /// Returns the current handle without tracking the read reactively.
    pub fn handle_untracked(&self) -> Option<TiptapEditorHandle> {
        self.0.get_untracked()
    }

    #[cfg(not(feature = "ssr"))]
    pub(crate) fn set_handle(&self, handle: TiptapEditorHandle) {
        self.0.set(Some(handle));
    }

    pub(crate) fn clear_handle(&self) {
        self.0.set(None);
    }

    pub(super) fn with_handle<T>(
        &self,
        f: impl FnOnce(&TiptapEditorHandle) -> Result<T, TiptapEditorError>,
    ) -> Result<T, TiptapEditorError> {
        self.0
            .get_untracked()
            .ok_or(TiptapEditorError::EditorUnavailable)
            .and_then(|handle| f(&handle))
    }
}

/// A handle to a live Tiptap editor instance.
///
/// The handle can be used to pull the current editor content in different formats or replace the
/// full document content.
///
/// It is safe to store this handle for as long as that concrete editor instance remains alive.
/// A handle becomes stale if the underlying editor is destroyed.
///
/// Internally, a handle is bound not only to the editor's public `id`, but also to a private
/// generation token assigned by the JS adapter when that concrete editor instance is created.
/// This prevents an old handle from accidentally talking to a newer editor that was later created
/// with the same DOM id after a destroy/recreate cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TiptapEditorHandle {
    pub(crate) id: String,
    /// Private instance generation used to reject stale handles after editor recreation.
    pub(crate) generation: u32,
}

impl TiptapEditorHandle {
    #[cfg(not(feature = "ssr"))]
    pub(crate) fn new(id: String, generation: u32) -> Self {
        Self { id, generation }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub(super) fn dispatch(
        &self,
        command: crate::protocol::EditorCommand,
    ) -> Result<(), TiptapEditorError> {
        crate::runtime::command(self.id.clone(), self.generation, command)
    }
}
