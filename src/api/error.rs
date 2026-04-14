use thiserror::Error;

/// Error type for editor operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TiptapEditorError {
    /// The requested editor instance is unavailable.
    #[error("the requested Tiptap editor instance is not available")]
    EditorUnavailable,

    /// An editor id is already in use.
    #[error("duplicate Tiptap editor id: {0}")]
    DuplicateEditorId(String),

    /// The editor could not be mounted.
    #[error("could not mount the Tiptap editor: {0}")]
    MountFailed(String),

    /// The provided editor content is invalid.
    #[error("invalid editor content: {0}")]
    InvalidContent(String),

    /// A JSON payload could not be parsed.
    #[error("could not deserialize Tiptap JSON: {0}")]
    InvalidJson(String),

    /// A bridge payload could not be parsed.
    #[error("could not deserialize Tiptap bridge payload: {0}")]
    InvalidBridgePayload(String),

    /// A command was rejected by the editor.
    #[error("editor command '{operation}' was rejected: {message}")]
    CommandRejected {
        /// The rejected operation name.
        operation: String,
        /// The rejection message.
        message: String,
    },

    /// An editor operation failed.
    #[error("editor operation '{operation}' failed: {message}")]
    OperationFailed {
        /// The failed operation name.
        operation: String,
        /// The failure message.
        message: String,
    },

    /// The browser bridge returned an error.
    #[error("Tiptap bridge error: {0}")]
    BridgeError(String),
}

/// Rootcause report type returned by public editor operations.
pub type TiptapEditorReport = rootcause::Report<TiptapEditorError>;

/// Result type returned by public editor operations.
pub type TiptapEditorResult<T> = Result<T, TiptapEditorReport>;

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn display_invalid_content_error() {
        assert_that!(TiptapEditorError::InvalidContent("bad json".to_owned()).to_string())
            .is_equal_to("invalid editor content: bad json".to_owned());
    }

    #[test]
    fn display_invalid_bridge_payload_error() {
        assert_that!(
            TiptapEditorError::InvalidBridgePayload("bad selection".to_owned()).to_string()
        )
        .is_equal_to("could not deserialize Tiptap bridge payload: bad selection".to_owned());
    }

    #[test]
    fn display_command_rejected_error() {
        assert_that!(
            TiptapEditorError::CommandRejected {
                operation: "toggle_bold".to_owned(),
                message: "selection required".to_owned(),
            }
            .to_string()
        )
        .is_equal_to("editor command 'toggle_bold' was rejected: selection required".to_owned());
    }

    #[test]
    fn display_operation_failed_error() {
        assert_that!(
            TiptapEditorError::OperationFailed {
                operation: "read_html".to_owned(),
                message: "editor crashed".to_owned(),
            }
            .to_string()
        )
        .is_equal_to("editor operation 'read_html' failed: editor crashed".to_owned());
    }
}
