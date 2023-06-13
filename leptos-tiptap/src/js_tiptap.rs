use tracing::error;
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::{TiptapEditorState, TiptapHeadingLevel, TiptapSelectionState};

mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(raw_module = "./js/tiptap.js")]
    extern "C" {
        pub fn create(
            id: String,
            content: String,
            editable: bool,
            onChange: &Closure<dyn Fn(String)>,
            onSelection: &Closure<dyn Fn(JsValue)>,
        );
        pub fn destroy(id: String);
        pub fn isEditable(id: String) -> bool;
        pub fn getHTML(id: String) -> JsValue;
        pub fn toggleHeading(id: String, level: i32) -> JsValue;
        pub fn setParagraph(id: String) -> JsValue;
        pub fn toggleBold(id: String) -> JsValue;
        pub fn toggleItalic(id: String) -> JsValue;
        pub fn toggleStrike(id: String) -> JsValue;
        pub fn toggleBlockquote(id: String) -> JsValue;
        pub fn toggleHighlight(id: String) -> JsValue;
        pub fn setTextAlignLeft(id: String) -> JsValue;
        pub fn setTextAlignCenter(id: String) -> JsValue;
        pub fn setTextAlignRight(id: String) -> JsValue;
        pub fn setTextAlignJustify(id: String) -> JsValue;
        pub fn setImage(id: String, src: String, alt: String, title: String) -> JsValue;
        pub fn getEditorState(id: String) -> JsValue;
        pub fn getSelectionState(id: String) -> JsValue;
    }
}

pub fn create(
    id: String,
    content: String,
    editable: bool,
    on_change: &Closure<dyn Fn(String)>,
    on_selection: &Closure<dyn Fn(JsValue)>,
) {
    js::create(id, content, editable, on_change, on_selection);
}

pub fn destroy(id: String) {
    js::destroy(id);
}

pub fn is_editable(id: String) -> bool {
    js::isEditable(id)
}

pub fn get_html(id: String) -> String {
    let value = js::getHTML(id);
    match value.as_string() {
        Some(string) => string,
        None => {
            error!(
                "JS function getHTML returned {:?}, which was not of the expected type: String",
                value
            );
            "error".to_owned()
        }
    }
}

pub fn toggle_heading(id: String, level: TiptapHeadingLevel) {
    js::toggleHeading(id, level.into());
}

pub fn set_paragraph(id: String) {
    js::setParagraph(id);
}

pub fn toggle_bold(id: String) {
    js::toggleBold(id);
}

pub fn toggle_italic(id: String) {
    js::toggleItalic(id);
}

pub fn toggle_strike(id: String) {
    js::toggleStrike(id);
}

pub fn toggle_blockquote(id: String) {
    js::toggleBlockquote(id);
}

pub fn toggle_highlight(id: String) {
    js::toggleHighlight(id);
}

pub fn set_text_align_left(id: String) {
    js::setTextAlignLeft(id);
}

pub fn set_text_align_center(id: String) {
    js::setTextAlignCenter(id);
}

pub fn set_text_align_right(id: String) {
    js::setTextAlignRight(id);
}

pub fn set_text_align_justify(id: String) {
    js::setTextAlignJustify(id);
}

pub fn set_image(id: String, src: String, alt: String, title: String) {
    js::setImage(id, src, alt, title);
}

pub fn get_editor_state(id: String) -> Result<TiptapEditorState, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(js::getEditorState(id))
}

pub fn get_selection_state(id: String) -> Result<TiptapSelectionState, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(js::getSelectionState(id))
}
