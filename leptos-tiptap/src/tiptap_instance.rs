use leptos::*;
use tracing::error;
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::{
    js_tiptap, TiptapContent, TiptapHeadingLevel, TiptapImageResource, TiptapSelectionState,
};

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
    SetImage(TiptapImageResource),
}

#[component]
pub fn TiptapInstance<C, S>(
    /// The ID of for this tiptap instance. Must be UNIQUE across ALL instances. You might want to use a UUID if uniqueness is otherwise not enforceable.
    #[prop(into)]
    id: String,

    /// Classes, optional.
    #[prop(optional, into)]
    class: Option<AttributeValue>,

    /// Styles, optional.
    #[prop(optional, into)]
    style: Option<AttributeValue>,

    /// Initial content of the editor.
    /// Note that the editor keeps an internal copy of this string and can solely work with that for an unlimited time.
    /// Changes made to this content by the user, by performing edits, is given out, but must not bre reflected through new values in this input signal.
    /// Considering that the content may be a very large String, cloning the whole content on every edit should be avoided!
    #[prop(into)]
    value: Signal<String>,

    /// Callback giving you the updated content of this tiptap instance.
    /// Every change to the content inside the editor is reflected back to you immediately through this callback.
    /// This will change / be configurable with: https://github.com/lpotthast/leptos-tiptap/issues/1
    set_value: C,

    /// This signal is your point of interaction with tiptap.
    /// Update this signal to a new value, and the action corresponding to the msg set will be executed.
    #[prop(into)]
    msg: Signal<TiptapInstanceMsg>,

    /// If set to true, the tiptap instance becomes un-editable. The instance reacts to changes of this signals value.
    #[prop(into)]
    disabled: MaybeSignal<bool>,

    /// Notifies you about a new selection. A selection changes, for example, if the cursor in the editor changes position, "selecting" a new element in the editor.
    /// Most actions, given by the changing `msg` signal values, are applied to the current selection.
    /// If a paragraph is selected and the H1 message is sent, that selected paragraph will be made an H1.
    on_selection_change: S,
) -> impl IntoView
where
    C: Fn(TiptapContent) + 'static,
    S: Fn(TiptapSelectionState) + 'static,
{
    let instance: NodeRef<leptos::html::Custom> = create_node_ref();

    let id = store_value(id);

    // This closure is passed on to the JS tiptap instance.
    // We expect this to be called whenever the INPUT in the editor changes.
    // We have to own this closure until the end of this components lifetime!
    let on_content_change_closure: StoredValue<Closure<dyn Fn(String)>> =
        store_value(Closure::wrap(Box::new(move |content| {
            set_value(TiptapContent::Html(content));
        }) as Box<dyn Fn(String)>));

    // This closure is passed on to the JS tiptap instance.
    // We expect this to be called whenever the SELECTION in the editor changes.
    // We have to own this closure until the end of this components lifetime!
    let on_selection_change_closure: StoredValue<Closure<dyn Fn(JsValue)>> = store_value(
        Closure::wrap(Box::new(move |selection_state_as_js_value| {
            on_selection_change(
                match serde_wasm_bindgen::from_value(selection_state_as_js_value) {
                    Ok(state) => state,
                    Err(err) => {
                        error!("Could not parse JsValue as TipTap state. Deserialization error: '{err}'. Falling back to default state.");
                        Default::default()
                    }
                },
            );
        }) as Box<dyn Fn(JsValue)>),
    );

    // The tiptap instance must be initialized EXACTLY ONCE through the tiptap JS API.
    let (initialized, set_initialized) = create_signal(false);
    create_effect(move |prev| {
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
                                set_initialized.set(true);
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

    on_cleanup(move || {
        js_tiptap::destroy(id.get_value());
    });

    // Talking to the tiptap instance here may ultimately trigger a content change.
    // This, and some other actions, may trigger callbacks reaching back to us using the closures above.
    // MAKE SURE that no signal is set in such a callback function so that this create_effect re-executes, as this might break it!
    // This is the reason why we handle on_content_change_closure and on_selection_change_closure without generating messages!
    // Besides that, TiptapInstanceMsg is a public enum and should / must only contain non-technical, non-destructive options.
    create_effect(move |_| {
        let msg = msg.get();
        if !initialized.get_untracked() {
            return;
        }
        match msg {
            TiptapInstanceMsg::Noop => {}
            TiptapInstanceMsg::H1 => {
                js_tiptap::toggle_heading(id.get_value(), TiptapHeadingLevel::H1);
            }
            TiptapInstanceMsg::H2 => {
                js_tiptap::toggle_heading(id.get_value(), TiptapHeadingLevel::H2);
            }
            TiptapInstanceMsg::H3 => {
                js_tiptap::toggle_heading(id.get_value(), TiptapHeadingLevel::H3);
            }
            TiptapInstanceMsg::H4 => {
                js_tiptap::toggle_heading(id.get_value(), TiptapHeadingLevel::H4);
            }
            TiptapInstanceMsg::H5 => {
                js_tiptap::toggle_heading(id.get_value(), TiptapHeadingLevel::H5);
            }
            TiptapInstanceMsg::H6 => {
                js_tiptap::toggle_heading(id.get_value(), TiptapHeadingLevel::H6);
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
        }
    });

    let disabled_memo = create_memo(move |_| disabled.get());

    create_effect(move |_| {
        let disabled = disabled_memo.get();
        if !initialized.get_untracked() {
            return;
        }
        js_tiptap::set_editable(id.get_value(), !disabled);
    });

    view! {
        <leptos-tiptap-instance
            node_ref=instance
            id=id.get_value()
            class=class
            style=style
            aria-disabled=move || disabled.get()
        />
    }
}
