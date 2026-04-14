use crate::protocol::{ContentFormat, ContentPayload, DocumentRequest, DocumentResponse};
use crate::runtime;

use super::{
    TiptapContent, TiptapEditorError, TiptapEditorHandle, TiptapEditorInstance, TiptapEditorResult,
    TiptapSetContentOptions,
};

impl TiptapEditorInstance {
    /// Returns the current editor document serialized as HTML.
    ///
    /// # Errors
    ///
    /// Returns an error when the JS bridge rejects the document request or
    /// returns a response in an unexpected format.
    pub fn get_html(&self) -> TiptapEditorResult<String> {
        extract_html_content(runtime::document(
            self.id.clone(),
            self.generation,
            DocumentRequest::GetContent {
                format: ContentFormat::Html,
            },
        )?)
    }

    /// Returns the current editor document serialized as `ProseMirror` JSON.
    ///
    /// # Errors
    ///
    /// Returns an error when the JS bridge rejects the document request or
    /// returns a response in an unexpected format.
    pub fn get_json(&self) -> TiptapEditorResult<serde_json::Value> {
        extract_json_content(runtime::document(
            self.id.clone(),
            self.generation,
            DocumentRequest::GetContent {
                format: ContentFormat::Json,
            },
        )?)
    }

    /// Replaces the current editor document content.
    ///
    /// # Errors
    ///
    /// Returns an error when the content cannot be converted for the bridge or
    /// when the JS bridge rejects the document replacement.
    pub fn set_content(&self, content: TiptapContent) -> TiptapEditorResult<()> {
        self.set_content_with_options(content, TiptapSetContentOptions::default())
    }

    /// Replaces the current editor document content with explicit Tiptap options.
    ///
    /// # Errors
    ///
    /// Returns an error when the content cannot be converted for the bridge or
    /// when the JS bridge rejects the document replacement.
    pub fn set_content_with_options(
        &self,
        content: TiptapContent,
        options: TiptapSetContentOptions,
    ) -> TiptapEditorResult<()> {
        let content = ContentPayload::try_from(content)?;
        expect_empty_document_response(runtime::document(
            self.id.clone(),
            self.generation,
            DocumentRequest::SetContent {
                content,
                options: Some(options.into()),
            },
        )?)
    }

    /// Replaces the current editor document with HTML content.
    ///
    /// # Errors
    ///
    /// Returns an error when the JS bridge rejects the document replacement.
    pub fn set_html(&self, content: impl Into<String>) -> TiptapEditorResult<()> {
        self.set_content(TiptapContent::html(content))
    }

    /// Replaces the current editor document with `ProseMirror` JSON content.
    ///
    /// # Errors
    ///
    /// Returns an error when the JS bridge rejects the document replacement.
    pub fn set_json(&self, content: impl Into<serde_json::Value>) -> TiptapEditorResult<()> {
        self.set_content(TiptapContent::json(content))
    }
}

impl TiptapEditorHandle {
    /// Returns the current editor document serialized as HTML.
    ///
    /// # Errors
    ///
    /// Returns an error when the handle has no ready editor instance or when
    /// the underlying instance request fails.
    pub fn get_html(&self) -> TiptapEditorResult<String> {
        self.with_instance(TiptapEditorInstance::get_html)
    }

    /// Returns the current editor document serialized as `ProseMirror` JSON.
    ///
    /// # Errors
    ///
    /// Returns an error when the handle has no ready editor instance or when
    /// the underlying instance request fails.
    pub fn get_json(&self) -> TiptapEditorResult<serde_json::Value> {
        self.with_instance(TiptapEditorInstance::get_json)
    }

    /// Replaces the current editor document content.
    ///
    /// # Errors
    ///
    /// Returns an error when the handle has no ready editor instance or when
    /// the underlying instance request fails.
    pub fn set_content(&self, content: TiptapContent) -> TiptapEditorResult<()> {
        self.set_content_with_options(content, TiptapSetContentOptions::default())
    }

    /// Replaces the current editor document content with explicit Tiptap options.
    ///
    /// # Errors
    ///
    /// Returns an error when the handle has no ready editor instance or when
    /// the underlying instance request fails.
    pub fn set_content_with_options(
        &self,
        content: TiptapContent,
        options: TiptapSetContentOptions,
    ) -> TiptapEditorResult<()> {
        self.with_instance(|instance| instance.set_content_with_options(content, options))
    }

    /// Replaces the current editor document with HTML content.
    ///
    /// # Errors
    ///
    /// Returns an error when the handle has no ready editor instance or when
    /// the underlying instance request fails.
    pub fn set_html(&self, content: impl Into<String>) -> TiptapEditorResult<()> {
        let content = content.into();
        self.with_instance(|instance| instance.set_html(content.clone()))
    }

    /// Replaces the current editor document with `ProseMirror` JSON content.
    ///
    /// # Errors
    ///
    /// Returns an error when the handle has no ready editor instance or when
    /// the underlying instance request fails.
    pub fn set_json(&self, content: impl Into<serde_json::Value>) -> TiptapEditorResult<()> {
        let content = content.into();
        self.with_instance(|instance| instance.set_json(content.clone()))
    }
}

fn extract_html_content(response: DocumentResponse) -> TiptapEditorResult<String> {
    match response {
        DocumentResponse::Content { content } => match content {
            ContentPayload::Html(content) => Ok(content),
            ContentPayload::Json(_) => Err(TiptapEditorError::BridgeError(
                "received JSON content for an HTML document request".to_owned(),
            )
            .into()),
        },
        DocumentResponse::Empty => Err(TiptapEditorError::BridgeError(
            "received an empty document response for an HTML document request".to_owned(),
        )
        .into()),
    }
}

fn extract_json_content(response: DocumentResponse) -> TiptapEditorResult<serde_json::Value> {
    match response {
        DocumentResponse::Content { content } => match content {
            ContentPayload::Json(content) => Ok(content),
            ContentPayload::Html(_) => Err(TiptapEditorError::BridgeError(
                "received HTML content for a JSON document request".to_owned(),
            )
            .into()),
        },
        DocumentResponse::Empty => Err(TiptapEditorError::BridgeError(
            "received an empty document response for a JSON document request".to_owned(),
        )
        .into()),
    }
}

fn expect_empty_document_response(response: DocumentResponse) -> TiptapEditorResult<()> {
    match response {
        DocumentResponse::Empty => Ok(()),
        DocumentResponse::Content { content } => {
            let format = match content {
                ContentPayload::Html(_) => ContentFormat::Html,
                ContentPayload::Json(_) => ContentFormat::Json,
            };

            Err(TiptapEditorError::BridgeError(format!(
                "received {format:?} content for a set_content document request"
            ))
            .into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ContentPayload;
    use assertr::prelude::*;

    #[test]
    fn html_requests_reject_json_document_responses() {
        let error = extract_html_content(DocumentResponse::Content {
            content: ContentPayload::Json(serde_json::json!({})),
        })
        .unwrap_err();

        assert_that!(error.into_current_context()).is_equal_to(TiptapEditorError::BridgeError(
            "received JSON content for an HTML document request".to_owned(),
        ));
    }
}
