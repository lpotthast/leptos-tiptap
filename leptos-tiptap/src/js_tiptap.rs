use cfg_if::cfg_if;
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::TiptapHeadingLevel;

#[cfg(not(feature = "ssr"))]
mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(raw_module = "/js/tiptap.js")]
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
        pub fn setEditable(id: String, editable: bool);
        pub fn getHTML(id: String) -> JsValue;
        pub fn toggleHeading(id: String, level: i32) -> JsValue;
        pub fn setParagraph(id: String) -> JsValue;
        pub fn toggleBold(id: String) -> JsValue;
        pub fn toggleItalic(id: String) -> JsValue;
        pub fn toggleStrike(id: String) -> JsValue;
        pub fn toggleBlockquote(id: String) -> JsValue;
        pub fn toggleHighlight(id: String) -> JsValue;
        pub fn toggleBulletList(id: String) -> JsValue;
        pub fn toggleOrderedList(id: String) -> JsValue;
        pub fn setTextAlignLeft(id: String) -> JsValue;
        pub fn setTextAlignCenter(id: String) -> JsValue;
        pub fn setTextAlignRight(id: String) -> JsValue;
        pub fn setTextAlignJustify(id: String) -> JsValue;
        pub fn setImage(id: String, src: String, alt: String, title: String) -> JsValue;
        pub fn setLink(id: String, href: String, target: String, rel: String) -> JsValue;
        pub fn toggleLink(id: String, href: String, target: String, rel: String) -> JsValue;
        pub fn unsetLink(id: String) -> JsValue;
        pub fn setYoutubeVideo(
            id: String,
            src: String,
            start: String,
            width: String,
            height: String,
        ) -> JsValue;
        pub fn getEditorState(id: String) -> JsValue;
        pub fn getSelectionState(id: String) -> JsValue;
    }
}

// TODO: Decide what to do with currently unused functionality.

pub fn create(
    id: String,
    content: String,
    editable: bool,
    on_change: &Closure<dyn Fn(String)>,
    on_selection: &Closure<dyn Fn(JsValue)>,
) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::create(id, content, editable, on_change, on_selection);
    } else {
        let _id = id;
        let _content = content;
        let _editable = editable;
        let _on_change = on_change;
        let _on_selection = on_selection;
    }}
}

pub fn destroy(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::destroy(id);
    } else {
        let _id = id;
    }}
}

// pub fn is_editable(id: String) -> bool {
//     js::isEditable(id)
// }

pub fn set_editable(id: String, editable: bool) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setEditable(id, editable);
    } else {
        let _id = id;
        let _editable = editable;
    }}
}

// pub fn get_html(id: String) -> String {
//     let value = js::getHTML(id);
//     match value.as_string() {
//         Some(string) => string,
//         None => {
//             error!(
//                 "JS function getHTML returned {:?}, which was not of the expected type: String",
//                 value
//             );
//             "error".to_owned()
//         }
//     }
// }

pub fn toggle_heading(id: String, level: TiptapHeadingLevel) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleHeading(id, level.into());
    } else {
        let _id = id;
        let _level = level;
    }}
}

pub fn set_paragraph(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setParagraph(id);
    } else {
        let _id = id;
    }}
}

pub fn toggle_bold(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleBold(id);
    } else {
        let _id = id;
    }}
}

pub fn toggle_italic(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleItalic(id);
    } else {
        let _id = id;
    }}
}

pub fn toggle_strike(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleStrike(id);
    } else {
        let _id = id;
    }}
}

pub fn toggle_blockquote(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleBlockquote(id);
    } else {
        let _id = id;
    }}
}

pub fn toggle_highlight(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleHighlight(id);
    } else {
        let _id = id;
    }}
}

pub fn toggle_bullet_list(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleBulletList(id);
    } else {
        let _id = id;
    }}
}

pub fn toggle_ordered_list(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleOrderedList(id);
    } else {
        let _id = id;
    }}
}

pub fn set_text_align_left(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setTextAlignLeft(id);
    } else {
        let _id = id;
    }}
}

pub fn set_text_align_center(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setTextAlignCenter(id);
    } else {
        let _id = id;
    }}
}

pub fn set_text_align_right(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setTextAlignRight(id);
    } else {
        let _id = id;
    }}
}

pub fn set_text_align_justify(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setTextAlignJustify(id);
    } else {
        let _id = id;
    }}
}

pub fn set_image(id: String, src: String, alt: String, title: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setImage(id, src, alt, title);
    } else {
        let _id = id;
        let _src = src;
        let _alt = alt;
        let _title = title;
    }}
}

pub fn set_link(id: String, href: String, target: String, rel: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setLink(id, href, target, rel);
    } else {
        let _id = id;
        let _href = href;
        let _target = target;
        let _rel = rel;
    }}
}

pub fn toggle_link(id: String, href: String, target: String, rel: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::toggleLink(id, href, target, rel);
    } else {
        let _id = id;
        let _href = href;
        let _target = target;
        let _rel = rel;
    }}
}

pub fn unset_link(id: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::unsetLink(id);
    } else {
        let _id = id;
    }}
}

pub fn set_youtube_video(id: String, src: String, start: String, width: String, height: String) {
    cfg_if! {if #[cfg(not(feature = "ssr"))] {
        js::setYoutubeVideo(id, src, start, width, height);
    } else {
        let _id = id;
        let _src = src;
        let _start = start;
        let _width = width;
        let _height = height;
    }}
}

// pub fn get_editor_state(id: String) -> Result<TiptapEditorState, serde_wasm_bindgen::Error> {
//     serde_wasm_bindgen::from_value(js::getEditorState(id))
// }

// pub fn get_selection_state(id: String) -> Result<TiptapSelectionState, serde_wasm_bindgen::Error> {
//     serde_wasm_bindgen::from_value(js::getSelectionState(id))
// }
