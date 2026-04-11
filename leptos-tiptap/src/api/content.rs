use crate::protocol::{ContentFormat, ContentPayload, DocumentRequest, DocumentResponse};
use crate::runtime;

use super::{
    TiptapContent, TiptapEditor, TiptapEditorError, TiptapEditorHandle, TiptapSetContentOptions,
};

impl TiptapEditorHandle {
    pub fn get_html(&self) -> Result<String, TiptapEditorError> {
        extract_html_content(runtime::document(
            self.id.clone(),
            self.generation,
            DocumentRequest::GetContent {
                format: ContentFormat::Html,
            },
        )?)
    }

    pub fn get_json(&self) -> Result<serde_json::Value, TiptapEditorError> {
        extract_json_content(runtime::document(
            self.id.clone(),
            self.generation,
            DocumentRequest::GetContent {
                format: ContentFormat::Json,
            },
        )?)
    }

    pub fn set_content(&self, content: TiptapContent) -> Result<(), TiptapEditorError> {
        self.set_content_with_options(content, TiptapSetContentOptions::default())
    }

    pub fn set_content_with_options(
        &self,
        content: TiptapContent,
        options: TiptapSetContentOptions,
    ) -> Result<(), TiptapEditorError> {
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

    pub fn set_html(&self, content: impl Into<String>) -> Result<(), TiptapEditorError> {
        self.set_content(TiptapContent::html(content))
    }

    pub fn set_json(&self, content: impl Into<serde_json::Value>) -> Result<(), TiptapEditorError> {
        self.set_content(TiptapContent::json(content))
    }
}

impl TiptapEditor {
    pub fn get_html(&self) -> Result<String, TiptapEditorError> {
        self.with_handle(TiptapEditorHandle::get_html)
    }

    pub fn get_json(&self) -> Result<serde_json::Value, TiptapEditorError> {
        self.with_handle(TiptapEditorHandle::get_json)
    }

    pub fn set_content(&self, content: TiptapContent) -> Result<(), TiptapEditorError> {
        self.set_content_with_options(content, TiptapSetContentOptions::default())
    }

    pub fn set_content_with_options(
        &self,
        content: TiptapContent,
        options: TiptapSetContentOptions,
    ) -> Result<(), TiptapEditorError> {
        self.with_handle(|handle| handle.set_content_with_options(content, options))
    }

    pub fn set_html(&self, content: impl Into<String>) -> Result<(), TiptapEditorError> {
        let content = content.into();
        self.with_handle(|handle| handle.set_html(content.clone()))
    }

    pub fn set_json(&self, content: impl Into<serde_json::Value>) -> Result<(), TiptapEditorError> {
        let content = content.into();
        self.with_handle(|handle| handle.set_json(content.clone()))
    }
}

fn extract_html_content(response: DocumentResponse) -> Result<String, TiptapEditorError> {
    match response {
        DocumentResponse::Content { content } => match content {
            ContentPayload::Html(content) => Ok(content),
            ContentPayload::Json(_) => Err(TiptapEditorError::BridgeError(
                "received JSON content for an HTML document request".to_owned(),
            )),
        },
        DocumentResponse::Empty => Err(TiptapEditorError::BridgeError(
            "received an empty document response for an HTML document request".to_owned(),
        )),
    }
}

fn extract_json_content(
    response: DocumentResponse,
) -> Result<serde_json::Value, TiptapEditorError> {
    match response {
        DocumentResponse::Content { content } => match content {
            ContentPayload::Json(content) => Ok(content),
            ContentPayload::Html(_) => Err(TiptapEditorError::BridgeError(
                "received HTML content for a JSON document request".to_owned(),
            )),
        },
        DocumentResponse::Empty => Err(TiptapEditorError::BridgeError(
            "received an empty document response for a JSON document request".to_owned(),
        )),
    }
}

fn expect_empty_document_response(response: DocumentResponse) -> Result<(), TiptapEditorError> {
    match response {
        DocumentResponse::Empty => Ok(()),
        DocumentResponse::Content { content } => {
            let format = match content {
                ContentPayload::Html(_) => ContentFormat::Html,
                ContentPayload::Json(_) => ContentFormat::Json,
            };

            Err(TiptapEditorError::BridgeError(format!(
                "received {:?} content for a set_content document request",
                format
            )))
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

        assert_that!(error).is_equal_to(TiptapEditorError::BridgeError(
            "received JSON content for an HTML document request".to_owned(),
        ));
    }
}
