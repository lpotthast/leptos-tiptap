use super::{TiptapEditorError, TiptapEditorResult};
use crate::protocol::EditorCommand;
use leptos::prelude::*;

/// Internal lifecycle of a [`TiptapEditorHandle`].
///
/// The handle exposes a coarse boolean readiness via [`TiptapEditorHandle::is_ready`], but tracks
/// the failure modes precisely so command attempts can return a specific
/// [`TiptapEditorError`] variant rather than a single coarse "unavailable" error.
#[derive(Debug, Clone, PartialEq, Eq)]
enum HandleState {
    NotReady,
    /// Only constructed in non-SSR builds where the JS bridge can mount an editor.
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    Ready(TiptapEditorInstance),
    Destroyed,
    CreateFailed,
}

/// A reactive editor handle that the `<TiptapEditor/>` component populates with a live
/// [`TiptapEditorInstance`] when the editor is ready, and updates on error or cleanup.
///
/// Create one with [`TiptapEditorHandle::new()`] and pass it to `<TiptapEditor/>` via the `handle`
/// prop. All command and content methods delegate to the inner instance, returning the matching
/// [`TiptapEditorError`] variant ([`NotReady`], [`Destroyed`], or [`CreateFailed`]) when the
/// editor is not currently ready.
///
/// A handle belongs to one logical editor. It may be retained while that editor is conditionally
/// unmounted and remounted, including while retrying a failed mount, but it must not be shared by
/// distinct or concurrently mounted editors. Each new session resets the handle to [`NotReady`]
/// before it can become ready, fail creation, or be destroyed.
///
/// `TiptapEditorHandle` is [`Copy`], so it can be freely captured in closures without cloning.
///
/// [`NotReady`]: TiptapEditorError::NotReady
/// [`Destroyed`]: TiptapEditorError::Destroyed
/// [`CreateFailed`]: TiptapEditorError::CreateFailed
#[derive(Clone, Copy)]
pub struct TiptapEditorHandle(RwSignal<HandleState>);

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
        Self(RwSignal::new(HandleState::NotReady))
    }

    /// Returns `true` when the underlying editor is ready.
    ///
    /// This is a reactive read; calling it inside an `Effect` or a derived signal
    /// will re-run when readiness changes.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.0.with(|state| matches!(state, HandleState::Ready(_)))
    }

    /// Returns the current live instance, if the editor is ready.
    ///
    /// This is a reactive read. Calling it inside an `Effect` or a derived signal will re-run when
    /// readiness changes.
    #[must_use]
    pub fn instance(&self) -> Option<TiptapEditorInstance> {
        self.0.with(|state| match state {
            HandleState::Ready(instance) => Some(instance.clone()),
            _ => None,
        })
    }

    /// Returns the current live instance without tracking the read reactively.
    #[must_use]
    pub fn instance_untracked(&self) -> Option<TiptapEditorInstance> {
        self.0.with_untracked(|state| match state {
            HandleState::Ready(instance) => Some(instance.clone()),
            _ => None,
        })
    }

    #[cfg(not(feature = "ssr"))]
    pub(crate) fn set_instance(&self, instance: TiptapEditorInstance) {
        self.0.set(HandleState::Ready(instance));
    }

    pub(crate) fn mark_not_ready(&self) {
        self.0.set(HandleState::NotReady);
    }

    pub(crate) fn mark_destroyed(&self) {
        self.0.set(HandleState::Destroyed);
    }

    pub(crate) fn mark_create_failed(&self) {
        self.0.set(HandleState::CreateFailed);
    }

    pub(super) fn with_instance<T>(
        &self,
        f: impl FnOnce(&TiptapEditorInstance) -> TiptapEditorResult<T>,
    ) -> TiptapEditorResult<T> {
        self.0.with_untracked(|state| match state {
            HandleState::Ready(instance) => f(instance),
            HandleState::NotReady => Err(TiptapEditorError::NotReady.into()),
            HandleState::Destroyed => Err(TiptapEditorError::Destroyed.into()),
            HandleState::CreateFailed => Err(TiptapEditorError::CreateFailed.into()),
        })
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

    pub(super) fn dispatch(&self, command: EditorCommand) -> TiptapEditorResult<()> {
        Ok(crate::runtime::command(
            self.id.clone(),
            self.generation,
            command,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    fn unavailable_error(handle: TiptapEditorHandle) -> TiptapEditorError {
        handle
            .with_instance::<()>(|_| Ok(()))
            .unwrap_err()
            .into_current_context()
    }

    #[test]
    fn reports_not_ready_before_an_instance_is_available() {
        Owner::new().with(|| {
            let handle = TiptapEditorHandle::new();

            assert_that!(unavailable_error(handle)).is_equal_to(TiptapEditorError::NotReady);
        });
    }

    #[test]
    fn reports_destroyed_after_cleanup() {
        Owner::new().with(|| {
            let handle = TiptapEditorHandle::new();
            handle.mark_destroyed();

            assert_that!(unavailable_error(handle)).is_equal_to(TiptapEditorError::Destroyed);
        });
    }

    #[test]
    fn reports_create_failed_after_mount_failure() {
        Owner::new().with(|| {
            let handle = TiptapEditorHandle::new();
            handle.mark_create_failed();

            assert_that!(unavailable_error(handle)).is_equal_to(TiptapEditorError::CreateFailed);
        });
    }
}
