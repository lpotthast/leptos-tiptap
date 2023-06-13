use leptos::*;
use serde::{Deserialize, Serialize};
use tracing::error;
use wasm_bindgen::prelude::Closure;

use crate::{js_tiptap, HeadingLevel, SelectionState};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ImageResource {
    // Example: image.png
    pub title: String,
    // Example: "An example image, ..."
    pub alt: String,
    // Example: https:://my-site.com/public/image.png
    pub url: String,
}

#[derive(Debug, Clone)]
pub enum TiptapInstanceMsg {
    Noop,

    /// Toggle "H1" for the current selection.
    H1,

    /// Toggle "H2" for the current selection.
    H2,

    /// Toggle "H3" for the current selection.
    H3,

    /// Toggle "H4" for the current selection.
    H4,

    /// Toggle "H5" for the current selection.
    H5,

    /// Toggle "H6" for the current selection.
    H6,

    /// Toggle "Paragraph" for the current selection.
    Paragraph,

    /// Toggle "Bold" for the current selection.
    Bold,

    /// Toggle "Italic" for the current selection.
    Italic,

    /// Toggle "Strike" for the current selection.
    Strike,

    /// Toggle "Blockquote" for the current selection.
    Blockquote,

    /// Toggle "Highlight" for the current selection.
    Highlight,

    /// Toggle "AlignLeft" for the current selection.
    AlignLeft,

    /// Toggle "AlignCenter" for the current selection.
    AlignCenter,

    /// Toggle "AlignRight" for the current selection.
    AlignRight,

    /// Toggle "AlignJustify" for the current selection.
    AlignJustify,

    /// Replace the current selection with an image.
    SetImage(ImageResource),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Content {
    pub content: String,
}

#[component]
pub fn TiptapInstance<C, S>(
    cx: Scope,
    #[prop(into)] id: String,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional, into)] style: Option<AttributeValue>,

    /// Initial content of the editor.
    /// Note that the editor keeps an internal copy of this string and can solely work with that for an unlimited time.
    /// Changes made to this content by the user, by performing edits, is given out, but must not bre reflected through new values in this input signal.
    /// Considering that the content may be a very large String, cloning the whole content on every edit should be avoided!
    #[prop(into)]
    value: Signal<String>,

    msg: ReadSignal<TiptapInstanceMsg>,

    /// If set to true, the tiptap instance becomes un-editable. TODO: Make this optional.
    #[prop(into)]
    disabled: MaybeSignal<bool>, // TODO: Handle changes!

    set_value: C,
    on_selection_change: S,
) -> impl IntoView
where
    C: Fn(Content) + 'static,
    S: Fn(SelectionState) + 'static,
{
    let instance: NodeRef<leptos::html::Div> = create_node_ref(cx);

    let id = store_value(cx, id);

    // This closure is passed on to the JS tiptap instance.
    // We expect this to be called whenever the INPUT in the editor changes.
    // We have to own this closure until the end of this components lifetime!
    let on_content_change_closure: StoredValue<Closure<dyn Fn(String)>> = store_value(
        cx,
        Closure::wrap(Box::new(move |content| {
            set_value(Content { content });
        }) as Box<dyn Fn(String)>),
    );

    // This closure is passed on to the JS tiptap instance.
    // We expect this to be called whenever the SELECTION in the editor changes.
    // We have to own this closure until the end of this components lifetime!
    let on_selection_change_closure: StoredValue<Closure<dyn Fn()>> = store_value(
        cx,
        Closure::wrap(Box::new(move || {
            on_selection_change(match js_tiptap::get_state(id.get_value()) {
                Ok(state) => state,
                Err(err) => {
                    error!("Could not parse JsValue as TipTap state. Deserialization error: '{err}'. Falling back to default state.");
                    Default::default()
                }
            });
        }) as Box<dyn Fn()>),
    );

    // The tiptap instance must be initialized EXACTLY ONCE through the tiptap JS API.
    create_effect(cx, move |prev| {
        if prev.is_none() || prev == Some(None) {
            return match instance.get() {
                Some(element) => {
                    element.on_mount(move |_element| {
                        on_content_change_closure.with_value(|on_change_closure| {
                            on_selection_change_closure.with_value(|on_selection_closure| {
                                js_tiptap::create(
                                    id.get_value(),
                                    value.get_untracked(),
                                    !disabled.get_untracked(),
                                    on_change_closure,
                                    on_selection_closure,
                                );
                            });
                        });
                    });
                    Some(())
                }
                None => None,
            };
        }
        None
    });

    // Talking to the tiptap instance here may ultimately trigger a content change.
    // This, and some other actions, may trigger callbacks reaching back to us using the closures above.
    // MAKE SURE that no signal is set in such a callback function so that this create_effect re-executes, as this might break it!
    // This is the reason why we handle on_content_change_closure and on_selection_change_closure without generating messages!
    // Besides that, TiptapInstanceMsg is a public enum and not only contain non-technical, non-destructive options.
    create_effect(cx, move |_| match msg.get() {
        TiptapInstanceMsg::Noop => {}
        TiptapInstanceMsg::H1 => {
            js_tiptap::toggle_heading(id.get_value(), HeadingLevel::H1);
        }
        TiptapInstanceMsg::H2 => {
            js_tiptap::toggle_heading(id.get_value(), HeadingLevel::H2);
        }
        TiptapInstanceMsg::H3 => {
            js_tiptap::toggle_heading(id.get_value(), HeadingLevel::H3);
        }
        TiptapInstanceMsg::H4 => {
            js_tiptap::toggle_heading(id.get_value(), HeadingLevel::H4);
        }
        TiptapInstanceMsg::H5 => {
            js_tiptap::toggle_heading(id.get_value(), HeadingLevel::H5);
        }
        TiptapInstanceMsg::H6 => {
            js_tiptap::toggle_heading(id.get_value(), HeadingLevel::H6);
        }
        TiptapInstanceMsg::Paragraph => {
            js_tiptap::set_paragraph(id.get_value());
        }
        TiptapInstanceMsg::Bold => {
            js_tiptap::toggle_bold(id.get_value());
        }
        TiptapInstanceMsg::Italic => {
            js_tiptap::toggle_italic(id.get_value());
        }
        TiptapInstanceMsg::Strike => {
            js_tiptap::toggle_strike(id.get_value());
        }
        TiptapInstanceMsg::Blockquote => {
            js_tiptap::toggle_blockquote(id.get_value());
        }
        TiptapInstanceMsg::Highlight => {
            js_tiptap::toggle_highlight(id.get_value());
        }
        TiptapInstanceMsg::AlignLeft => {
            js_tiptap::set_text_align_left(id.get_value());
        }
        TiptapInstanceMsg::AlignCenter => {
            js_tiptap::set_text_align_center(id.get_value());
        }
        TiptapInstanceMsg::AlignRight => {
            js_tiptap::set_text_align_right(id.get_value());
        }
        TiptapInstanceMsg::AlignJustify => {
            js_tiptap::set_text_align_justify(id.get_value());
        }
        TiptapInstanceMsg::SetImage(resource) => {
            js_tiptap::set_image(id.get_value(), resource.url, resource.alt, resource.title);
        }
    });

    view! {cx,
        <div
            id=id.get_value()
            class=class
            style=style
            node_ref=instance
        ></div>
    }
}
