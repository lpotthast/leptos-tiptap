use leptos::prelude::*;
use wasm_bindgen::{prelude::Closure, JsValue};

use send_wrapper::SendWrapper;

use crate::{
    js_tiptap, TiptapContent, TiptapHeadingLevel, TiptapImageResource, TiptapLinkResource,
    TiptapSelectionState, TiptapYoutubeVideoResource,
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

    /// Replace the current selection with a link.
    SetLink(TiptapLinkResource),

    /// Toggle if the current selection is a link
    ToggleLink(TiptapLinkResource),

    /// Remove the link from the selection
    UnsetLink(),

    /// Replace the current selection with an embedded youtube video
    SetYoutubeVideo(TiptapYoutubeVideoResource),
}

#[component]
pub fn TiptapInstance(
    /// The ID of for this tiptap instance. Must be UNIQUE across ALL instances. You might want to use a UUID (v4 or v7) if uniqueness is otherwise not enforceable.
    /// This may be a signal to support SSR. If the client should regenerate this ID, the old instance is removed (if one existed) and a new instance is created.
    #[prop(into)]
    id: Signal<String>,

    /// Initial content of the editor.
    /// Note that the editor keeps an internal copy of this string and can solely work with that for an unlimited time.
    /// Changes made to this content by the user, by performing edits, is given out, but must not bre reflected through new values in this input signal.
    /// Considering that the content may be a very large String, cloning the whole content on every edit should be avoided!
    #[prop(into)]
    value: Signal<String>,

    /// Callback giving you the updated content of this tiptap instance.
    /// Every change to the content inside the editor is reflected back to you immediately through this callback.
    /// This will change / be configurable with: https://github.com/lpotthast/leptos-tiptap/issues/1
    #[prop(into)]
    set_value: Callback<TiptapContent>,

    /// This signal is your point of interaction with tiptap.
    /// Update this signal to a new value, and the action corresponding to the msg set will be executed.
    #[prop(into)]
    msg: Signal<TiptapInstanceMsg>,

    /// If set to true, the tiptap instance becomes un-editable. The instance reacts to changes of this signals value.
    #[prop(into)]
    disabled: Signal<bool>,

    /// Notifies you about a new selection. A selection changes, for example, if the cursor in the editor changes position, "selecting" a new element in the editor.
    /// Most actions, given by the changing `msg` signal values, are applied to the current selection.
    /// If a paragraph is selected and the H1 message is sent, that selected paragraph will be made an H1.
    #[prop(into)]
    on_selection_change: Callback<TiptapSelectionState>,
) -> impl IntoView {
    let instance = NodeRef::new();

    let id: Signal<String> = Signal::derive(move || id.get());

    // Make this component SSR compatible by moving all JS interaction inside an effect.
    Effect::new(move |old_id: Option<String>| {
        // Rerun this effect whenever the ID should change!
        if let Some(old_id) = &old_id {
            js_tiptap::destroy(old_id.clone());
        }

        // This closure is passed on to the JS tiptap instance.
        // We expect this to be called whenever the INPUT in the editor changes.
        // We have to own this closure until the end of this components lifetime!
        let on_content_change_closure =
            StoredValue::new(SendWrapper::new(Closure::wrap(Box::new(move |content| {
                set_value.run(TiptapContent::Html(content));
            })
                as Box<dyn Fn(String)>)));

        // This closure is passed on to the JS tiptap instance.
        // We expect this to be called whenever the SELECTION in the editor changes.
        // We have to own this closure until the end of this components lifetime!
        let on_selection_change_closure = StoredValue::new(SendWrapper::new(Closure::wrap(
            Box::new(move |selection_state_as_js_value| {
                on_selection_change.run(
                    match serde_wasm_bindgen::from_value(selection_state_as_js_value) {
                        Ok(state) => state,
                        Err(err) => {
                            tracing::error!("Could not parse JsValue as TipTap state. Deserialization error: '{err}'. Falling back to default state.");
                            Default::default()
                        }
                    },
                );
            }) as Box<dyn Fn(JsValue)>,
        )));

        // The tiptap instance must be initialized EXACTLY ONCE through the tiptap JS API.
        let (initialized, set_initialized) = signal(false);
        Effect::new(move |_| {
            if !initialized.get_untracked() && instance.get().is_some() {
                js_tiptap::create(
                    id.get_untracked(),
                    value.get_untracked(),
                    !disabled.get_untracked(),
                    &on_content_change_closure.read_value(),
                    &on_selection_change_closure.read_value(),
                );
                set_initialized.set(true);
            }
        });

        Effect::new(move |_| {
            let id = id.get();
            // Push an additional on_cleanup handler every time the id changes. Accessing the last id, which would be what we want, lead to panics as the underlying data is already destroyed.
            // TODO: Why does it not work to access `id.get_untracked()` inside the `on_cleanup` handler?
            on_cleanup(move || {
                js_tiptap::destroy(id.clone());
            });
        });

        // Talking to the tiptap instance here may ultimately trigger a content change.
        // This, and some other actions, may trigger callbacks reaching back to us using the closures above.
        // MAKE SURE that no signal is set in such a callback function so that this create_effect re-executes, as this might break it!
        // This is the reason why we handle on_content_change_closure and on_selection_change_closure without generating messages!
        // Besides that, TiptapInstanceMsg is a public enum and should / must only contain non-technical, non-destructive options.
        Effect::new(move |_| {
            let msg = msg.get();
            if !initialized.get_untracked() {
                return;
            }
            let id = id.get_untracked();
            match msg {
                TiptapInstanceMsg::Noop => {}
                TiptapInstanceMsg::H1 => {
                    js_tiptap::toggle_heading(id, TiptapHeadingLevel::H1);
                }
                TiptapInstanceMsg::H2 => {
                    js_tiptap::toggle_heading(id, TiptapHeadingLevel::H2);
                }
                TiptapInstanceMsg::H3 => {
                    js_tiptap::toggle_heading(id, TiptapHeadingLevel::H3);
                }
                TiptapInstanceMsg::H4 => {
                    js_tiptap::toggle_heading(id, TiptapHeadingLevel::H4);
                }
                TiptapInstanceMsg::H5 => {
                    js_tiptap::toggle_heading(id, TiptapHeadingLevel::H5);
                }
                TiptapInstanceMsg::H6 => {
                    js_tiptap::toggle_heading(id, TiptapHeadingLevel::H6);
                }
                TiptapInstanceMsg::Paragraph => {
                    js_tiptap::set_paragraph(id);
                }
                TiptapInstanceMsg::Bold => {
                    js_tiptap::toggle_bold(id);
                }
                TiptapInstanceMsg::Italic => {
                    js_tiptap::toggle_italic(id);
                }
                TiptapInstanceMsg::Strike => {
                    js_tiptap::toggle_strike(id);
                }
                TiptapInstanceMsg::Blockquote => {
                    js_tiptap::toggle_blockquote(id);
                }
                TiptapInstanceMsg::Highlight => {
                    js_tiptap::toggle_highlight(id);
                }
                TiptapInstanceMsg::AlignLeft => {
                    js_tiptap::set_text_align_left(id);
                }
                TiptapInstanceMsg::AlignCenter => {
                    js_tiptap::set_text_align_center(id);
                }
                TiptapInstanceMsg::AlignRight => {
                    js_tiptap::set_text_align_right(id);
                }
                TiptapInstanceMsg::AlignJustify => {
                    js_tiptap::set_text_align_justify(id);
                }
                TiptapInstanceMsg::SetImage(resource) => {
                    js_tiptap::set_image(id, resource.url, resource.alt, resource.title);
                }
                TiptapInstanceMsg::SetLink(resource) => {
                    js_tiptap::set_link(id, resource.href, resource.target, resource.rel);
                }
                TiptapInstanceMsg::ToggleLink(resource) => {
                    js_tiptap::toggle_link(id, resource.href, resource.target, resource.rel);
                }
                TiptapInstanceMsg::UnsetLink() => {
                    js_tiptap::unset_link(id);
                }
                TiptapInstanceMsg::SetYoutubeVideo(resource) => {
                    js_tiptap::set_youtube_video(
                        id,
                        resource.src,
                        resource.start,
                        resource.width,
                        resource.height,
                    );
                }
            }
        });

        let disabled_memo = Memo::new(move |_| disabled.get());

        Effect::new(move |_| {
            let disabled = disabled_memo.get();
            if !initialized.get_untracked() {
                return;
            }
            js_tiptap::set_editable(id.get_untracked(), !disabled);
        });

        id.get_untracked()
    });

    view! {
        <leptos-tiptap-instance
            node_ref=instance
            id=move || id.get()
            aria-disabled=move || disabled.get()
        />
    }
}
