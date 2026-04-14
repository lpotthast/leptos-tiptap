use super::TiptapEditorError;
use leptos::prelude::*;

/// A reactive editor handle that the `<TiptapEditor/>` component populates with a live
/// [`TiptapEditorInstance`] when the editor is ready, and clears on error or cleanup.
///
/// Create one with [`TiptapEditorHandle::new()`] and pass it to `<TiptapEditor/>` via the `editor`
/// prop. All command and content methods delegate to the inner instance, returning
/// [`TiptapEditorError::EditorUnavailable`] when the editor is not yet ready.
///
/// `TiptapEditorHandle` is [`Copy`], so it can be freely captured in closures without cloning.
#[derive(Clone, Copy)]
pub struct TiptapEditorHandle(RwSignal<Option<TiptapEditorInstance>>);

/// Compatibility alias for the user-held reactive editor handle.
///
/// Use [`TiptapEditorHandle`] in new code. The alias is kept so old type annotations such as
/// `TiptapEditor::new()` can migrate independently from component call sites that also use the
/// `TiptapEditor` name.
pub type TiptapEditor = TiptapEditorHandle;

impl Default for TiptapEditorHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl TiptapEditorHandle {
    /// Creates a new, empty editor handle.
    ///
    /// Must be called within a reactive owner scope (e.g. inside a component body).
    #[must_use]
    pub fn new() -> Self {
        Self(RwSignal::new(None))
    }

    /// Returns `true` when the underlying editor is ready.
    ///
    /// This is a reactive read; calling it inside an `Effect` or a derived signal
    /// will re-run when readiness changes.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.0.with(std::option::Option::is_some)
    }

    /// Returns the current live instance, if the editor is ready.
    ///
    /// This is a reactive read; calling it inside an `Effect` or a derived signal
    /// will re-run when readiness changes.
    #[must_use]
    pub fn instance(&self) -> Option<TiptapEditorInstance> {
        self.0.get()
    }

    /// Returns the current live instance without tracking the read reactively.
    #[must_use]
    pub fn instance_untracked(&self) -> Option<TiptapEditorInstance> {
        self.0.get_untracked()
    }

    #[cfg(not(feature = "ssr"))]
    pub(crate) fn set_instance(&self, instance: TiptapEditorInstance) {
        self.0.set(Some(instance));
    }

    pub(crate) fn clear_instance(&self) {
        self.0.set(None);
    }

    pub(super) fn with_instance<T>(
        &self,
        f: impl FnOnce(&TiptapEditorInstance) -> Result<T, TiptapEditorError>,
    ) -> Result<T, TiptapEditorError> {
        self.0
            .get_untracked()
            .ok_or(TiptapEditorError::EditorUnavailable)
            .and_then(|instance| f(&instance))
    }
}

/// A live Tiptap editor instance.
///
/// The instance can be used to pull the current editor content in different formats or replace the
/// full document content. Most callers should store and use [`TiptapEditorHandle`] instead, and
/// only work with this concrete instance when they need its stable editor id.
///
/// It is safe to store this instance for as long as that concrete editor instance remains alive.
/// It becomes stale if the underlying editor is destroyed.
///
/// Internally, an instance is bound not only to the editor's public `id`, but also to a private
/// generation token assigned by the JS adapter when that concrete editor instance is created.
/// This prevents an old instance from accidentally talking to a newer editor that was later created
/// with the same DOM id after a destroy/recreate cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TiptapEditorInstance {
    pub(crate) id: String,
    /// Private instance generation used to reject stale instances after editor recreation.
    pub(crate) generation: u32,
}

impl TiptapEditorInstance {
    #[cfg(not(feature = "ssr"))]
    pub(crate) fn new(id: String, generation: u32) -> Self {
        Self { id, generation }
    }

    /// Returns the stable public editor id.
    #[must_use]
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
