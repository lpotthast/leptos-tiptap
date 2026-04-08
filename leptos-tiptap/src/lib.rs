use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

mod js_tiptap;
mod tiptap_instance;

pub use tiptap_instance::TiptapInstance;

/// Editor content payload.
///
/// `Html` contains editor content as HTML.
/// `Json` contains the Tiptap/ProseMirror JSON document as structured data.
#[derive(Debug, PartialEq, Clone)]
pub enum TiptapContent {
    Html(String),
    Json(serde_json::Value),
}

impl TiptapContent {
    pub fn html(content: impl Into<String>) -> Self {
        Self::Html(content.into())
    }

    pub fn json(content: impl Into<serde_json::Value>) -> Self {
        Self::Json(content.into())
    }

    pub fn json_str(content: impl AsRef<str>) -> Result<Self, serde_json::Error> {
        serde_json::from_str(content.as_ref()).map(Self::Json)
    }

    pub(crate) fn into_payload(self) -> js_tiptap::ContentPayload {
        match self {
            Self::Html(content) => js_tiptap::ContentPayload {
                format: js_tiptap::ContentFormat::Html,
                value: content,
            },
            Self::Json(content) => js_tiptap::ContentPayload {
                format: js_tiptap::ContentFormat::Json,
                value: serde_json::to_string(&content).unwrap_or_else(|err| {
                    tracing::error!(
                        "Could not serialize TipTap JSON content for JS interop. Error: '{err}'. Falling back to null."
                    );
                    "null".to_owned()
                }),
            },
        }
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
    id: String,
    /// Private instance generation used to reject stale handles after editor recreation.
    generation: u32,
}

impl TiptapEditorHandle {
    pub(crate) fn new(id: String, generation: u32) -> Self {
        Self { id, generation }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn get_html(&self) -> Result<String, TiptapEditorError> {
        extract_html_content(js_tiptap::document(
            self.id.clone(),
            self.generation,
            js_tiptap::DocumentRequest::GetContent {
                format: js_tiptap::ContentFormat::Html,
            },
        )?)
    }

    pub fn get_json(&self) -> Result<serde_json::Value, TiptapEditorError> {
        extract_json_content(js_tiptap::document(
            self.id.clone(),
            self.generation,
            js_tiptap::DocumentRequest::GetContent {
                format: js_tiptap::ContentFormat::Json,
            },
        )?)
    }

    pub fn set_content(&self, content: TiptapContent) -> Result<(), TiptapEditorError> {
        expect_empty_document_response(js_tiptap::document(
            self.id.clone(),
            self.generation,
            js_tiptap::DocumentRequest::SetContent {
                content: content.into_payload(),
            },
        )?)
    }

    pub fn set_html(&self, content: impl Into<String>) -> Result<(), TiptapEditorError> {
        self.set_content(TiptapContent::html(content))
    }

    pub fn set_json(&self, content: impl Into<serde_json::Value>) -> Result<(), TiptapEditorError> {
        self.set_content(TiptapContent::json(content))
    }

    fn dispatch(&self, command: js_tiptap::EditorCommand) -> Result<(), TiptapEditorError> {
        js_tiptap::command(self.id.clone(), self.generation, command)
    }

    pub fn toggle_heading(&self, level: TiptapHeadingLevel) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleHeading {
            level: level.into(),
        })
    }

    pub fn set_paragraph(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::SetParagraph)
    }

    pub fn toggle_bold(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleBold)
    }

    pub fn toggle_italic(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleItalic)
    }

    pub fn toggle_strike(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleStrike)
    }

    pub fn toggle_blockquote(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleBlockquote)
    }

    pub fn toggle_highlight(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleHighlight)
    }

    pub fn toggle_bullet_list(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleBulletList)
    }

    pub fn toggle_ordered_list(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleOrderedList)
    }

    pub fn set_text_align_left(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::SetTextAlignLeft)
    }

    pub fn set_text_align_center(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::SetTextAlignCenter)
    }

    pub fn set_text_align_right(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::SetTextAlignRight)
    }

    pub fn set_text_align_justify(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::SetTextAlignJustify)
    }

    pub fn set_image(&self, image: TiptapImageResource) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::SetImage {
            src: image.url,
            alt: image.alt,
            title: image.title,
        })
    }

    pub fn set_link(&self, link: TiptapLinkResource) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::SetLink {
            href: link.href,
            target: link.target,
            rel: link.rel,
        })
    }

    pub fn toggle_link(&self, link: TiptapLinkResource) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::ToggleLink {
            href: link.href,
            target: link.target,
            rel: link.rel,
        })
    }

    pub fn unset_link(&self) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::UnsetLink)
    }

    pub fn set_youtube_video(
        &self,
        video: TiptapYoutubeVideoResource,
    ) -> Result<(), TiptapEditorError> {
        self.dispatch(js_tiptap::EditorCommand::SetYoutubeVideo {
            src: video.src,
            start: video.start,
            width: video.width,
            height: video.height,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TiptapEditorError {
    EditorUnavailable,
    MountFailed(String),
    InvalidContent(String),
    InvalidJson(String),
    CommandRejected { operation: String, message: String },
    OperationFailed { operation: String, message: String },
    BridgeError(String),
}

impl fmt::Display for TiptapEditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EditorUnavailable => {
                write!(f, "the requested Tiptap editor instance is not available")
            }
            Self::MountFailed(err) => write!(f, "could not mount the Tiptap editor: {err}"),
            Self::InvalidContent(err) => write!(f, "invalid editor content: {err}"),
            Self::InvalidJson(err) => write!(f, "could not deserialize Tiptap JSON: {err}"),
            Self::CommandRejected { operation, message } => {
                write!(f, "editor command '{operation}' was rejected: {message}")
            }
            Self::OperationFailed { operation, message } => {
                write!(f, "editor operation '{operation}' failed: {message}")
            }
            Self::BridgeError(err) => write!(f, "Tiptap bridge error: {err}"),
        }
    }
}

impl Error for TiptapEditorError {}

fn extract_html_content(
    response: js_tiptap::DocumentResponse,
) -> Result<String, TiptapEditorError> {
    match response {
        js_tiptap::DocumentResponse::Content { content } => match content.format {
            js_tiptap::ContentFormat::Html => Ok(content.value),
            js_tiptap::ContentFormat::Json => Err(TiptapEditorError::BridgeError(
                "received JSON content for an HTML document request".to_owned(),
            )),
        },
        js_tiptap::DocumentResponse::Empty => Err(TiptapEditorError::BridgeError(
            "received an empty document response for an HTML document request".to_owned(),
        )),
    }
}

fn extract_json_content(
    response: js_tiptap::DocumentResponse,
) -> Result<serde_json::Value, TiptapEditorError> {
    match response {
        js_tiptap::DocumentResponse::Content { content } => match content.format {
            js_tiptap::ContentFormat::Json => serde_json::from_str(&content.value)
                .map_err(|err| TiptapEditorError::InvalidJson(err.to_string())),
            js_tiptap::ContentFormat::Html => Err(TiptapEditorError::BridgeError(
                "received HTML content for a JSON document request".to_owned(),
            )),
        },
        js_tiptap::DocumentResponse::Empty => Err(TiptapEditorError::BridgeError(
            "received an empty document response for a JSON document request".to_owned(),
        )),
    }
}

fn expect_empty_document_response(
    response: js_tiptap::DocumentResponse,
) -> Result<(), TiptapEditorError> {
    match response {
        js_tiptap::DocumentResponse::Empty => Ok(()),
        js_tiptap::DocumentResponse::Content { content } => {
            Err(TiptapEditorError::BridgeError(format!(
                "received {:?} content for a set_content document request",
                content.format
            )))
        }
    }
}

/// State of the current selection.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TiptapSelectionState {
    /// 'true' if the cursor is in a h1.
    pub h1: bool,

    /// 'true' if the cursor is in a h2.
    pub h2: bool,

    /// 'true' if the cursor is in a h3.
    pub h3: bool,

    /// 'true' if the cursor is in a h4.
    pub h4: bool,

    /// 'true' if the cursor is in a h5.
    pub h5: bool,

    /// 'true' if the cursor is in a h6.
    pub h6: bool,

    /// 'true' if the cursor is in a paragraph.
    pub paragraph: bool,

    /// 'true' if the cursor is in a bold text segment.
    pub bold: bool,

    /// 'true' if the cursor is in an italic text segment.
    pub italic: bool,

    /// 'true' if the cursor is in a strikethrough text segment.
    pub strike: bool,

    /// 'true' if the cursor is in a blockquote.
    pub blockquote: bool,

    /// 'true' if the cursor is in a highlighted text segment.
    pub highlight: bool,

    /// 'true' if the cursor is in a bullet-list.
    pub bullet_list: bool,

    /// 'true' if the cursor is in an ordered-list.
    pub ordered_list: bool,

    /// 'true' if the cursor is in a left-aligned text segment.
    pub align_left: bool,

    /// 'true' if the cursor is in a center-aligned text segment.
    pub align_center: bool,

    /// 'true' if the cursor is in a right-aligned text segment.
    pub align_right: bool,

    /// 'true' if the cursor is in a justify-aligned text segment.
    pub align_justify: bool,

    /// 'true' if the cursor is in a link.
    pub link: bool,

    /// 'true' if the cursor is on en embedded YouTube video.
    pub youtube: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiptapHeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl From<TiptapHeadingLevel> for i32 {
    fn from(val: TiptapHeadingLevel) -> Self {
        match val {
            TiptapHeadingLevel::H1 => 1,
            TiptapHeadingLevel::H2 => 2,
            TiptapHeadingLevel::H3 => 3,
            TiptapHeadingLevel::H4 => 4,
            TiptapHeadingLevel::H5 => 5,
            TiptapHeadingLevel::H6 => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapImageResource {
    /// Example: image.png
    pub title: String,

    /// Example: "An example image, ..."
    pub alt: String,

    /// Example: https:://my-site.com/public/image.png
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapLinkResource {
    /// Example: https:://my-site.com
    pub href: String,

    /// Example: "_blank", specifies where to open the linked document
    pub target: Option<String>,

    /// Example: "alternate"
    pub rel: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TiptapYoutubeVideoResource {
    /// Example: https://www.youtube.com/embed/dQw4w9WgXcQ?si=6LwJzVo1t8hpLywC
    pub src: String,

    /// Example: 0, specifies when to start the video
    pub start: Option<u32>,

    /// Example: 640
    pub width: Option<u32>,

    /// Example: 480
    pub height: Option<u32>,
}

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
    fn display_command_rejected_error() {
        assert_that!(TiptapEditorError::CommandRejected {
            operation: "toggle_bold".to_owned(),
            message: "selection required".to_owned(),
        }
        .to_string())
        .is_equal_to("editor command 'toggle_bold' was rejected: selection required".to_owned());
    }

    #[test]
    fn display_operation_failed_error() {
        assert_that!(TiptapEditorError::OperationFailed {
            operation: "read_html".to_owned(),
            message: "editor crashed".to_owned(),
        }
        .to_string())
        .is_equal_to("editor operation 'read_html' failed: editor crashed".to_owned());
    }

    #[test]
    fn html_requests_reject_json_document_responses() {
        let error = extract_html_content(js_tiptap::DocumentResponse::Content {
            content: js_tiptap::ContentPayload {
                format: js_tiptap::ContentFormat::Json,
                value: "{}".to_owned(),
            },
        })
        .unwrap_err();

        assert_that!(error).is_equal_to(TiptapEditorError::BridgeError(
            "received JSON content for an HTML document request".to_owned(),
        ));
    }

    #[test]
    fn set_content_rejects_non_empty_document_responses() {
        let error = expect_empty_document_response(js_tiptap::DocumentResponse::Content {
            content: js_tiptap::ContentPayload {
                format: js_tiptap::ContentFormat::Html,
                value: "<p>hello</p>".to_owned(),
            },
        })
        .unwrap_err();

        assert_that!(error).is_equal_to(TiptapEditorError::BridgeError(
            "received Html content for a set_content document request".to_owned(),
        ));
    }
}
