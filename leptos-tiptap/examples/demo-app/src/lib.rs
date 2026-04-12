use leptos::prelude::*;
use leptos::serde_json;
use leptos_tiptap::{
    TiptapContent, TiptapEditor, TiptapExtension, TiptapHeadingLevel, TiptapLinkResource,
    TiptapSelectionState, TiptapTextAlign, TiptapYoutubeVideoResource,
};

#[component]
pub fn DemoApp() -> impl IntoView {
    let editor = TiptapEditor::new();
    let (selection, set_selection) = signal(TiptapSelectionState::default());
    let (disabled, set_disabled) = signal(false);
    let (html_output, set_html_output) = signal(String::new());
    let (json_output, set_json_output) = signal(String::new());

    view! {
        <section id="html-demo">
            <h2>"HTML editor"</h2>

            <button on:click=move |_| set_disabled.set(!disabled.get())>"Disabled: " { move || disabled.get() }</button>
            <button
                on:click=move |_| replace_editor_content(&editor, set_html_output, set_json_output)
                disabled=move || !editor.is_ready()
            >
                "Replace content"
            </button>
            <button on:click=move |_| { let _ = editor.toggle_heading(TiptapHeadingLevel::H1); }>"H1"</button>
            <button on:click=move |_| { let _ = editor.toggle_heading(TiptapHeadingLevel::H2); }>"H2"</button>
            <button on:click=move |_| { let _ = editor.toggle_heading(TiptapHeadingLevel::H3); }>"H3"</button>
            <button on:click=move |_| { let _ = editor.toggle_heading(TiptapHeadingLevel::H4); }>"H4"</button>
            <button on:click=move |_| { let _ = editor.toggle_heading(TiptapHeadingLevel::H5); }>"H5"</button>
            <button on:click=move |_| { let _ = editor.toggle_heading(TiptapHeadingLevel::H6); }>"H6"</button>
            <button on:click=move |_| { let _ = editor.set_paragraph(); }>"Paragraph"</button>
            <button on:click=move |_| { let _ = editor.toggle_bold(); }>"Bold"</button>
            <button on:click=move |_| { let _ = editor.toggle_italic(); }>"Italic"</button>
            <button on:click=move |_| { let _ = editor.toggle_strike(); }>"Strike"</button>
            <button on:click=move |_| { let _ = editor.toggle_blockquote(); }>"Blockquote"</button>
            <button on:click=move |_| { let _ = editor.toggle_highlight(None); }>"Highlight"</button>
            <button on:click=move |_| { let _ = editor.toggle_bullet_list(); }>"BulletList"</button>
            <button on:click=move |_| { let _ = editor.toggle_ordered_list(); }>"OrderedList"</button>
            <button on:click=move |_| { let _ = editor.set_text_align(TiptapTextAlign::Left); }>"AlignLeft"</button>
            <button on:click=move |_| { let _ = editor.set_text_align(TiptapTextAlign::Center); }>"AlignCenter"</button>
            <button on:click=move |_| { let _ = editor.set_text_align(TiptapTextAlign::Right); }>"AlignRight"</button>
            <button on:click=move |_| { let _ = editor.set_text_align(TiptapTextAlign::Justify); }>"AlignJustify"</button>
            <button on:click=move |_| {
                let _ = editor.set_link(TiptapLinkResource {
                    href: "https://www.google.com/".to_string(),
                    target: Some("_blank".to_string()),
                    rel: Some("alternate".to_string()),
                    class: None,
                });
            }>
                "Set link"
            </button>
            <button on:click=move |_| {
                let _ = editor.toggle_link(TiptapLinkResource {
                    href: "https://www.google.com/".to_string(),
                    target: Some("_blank".to_string()),
                    rel: Some("alternate".to_string()),
                    class: None,
                });
            }>
                "Toggle link"
            </button>
            <button on:click=move |_| { let _ = editor.unset_link(); }>"Unset link"</button>
            <button on:click=move |_| {
                let _ = editor.set_youtube_video(TiptapYoutubeVideoResource {
                    src: "https://www.youtube.com/embed/dQw4w9WgXcQ?si=6LwJzVo1t8hpLywC".to_string(),
                    start: Some(0),
                    width: Some(640),
                    height: Some(480),
                });
            }>
                "Toggle YouTube video"
            </button>

            <TiptapEditor
                editor=editor
                id="id"
                disabled=disabled
                initial_content=initial_html_content()
                placeholder="Start typing here..."
                on_ready=move |_| sync_editor_outputs(&editor, set_html_output, set_json_output)
                on_change=move |_| sync_editor_outputs(&editor, set_html_output, set_json_output)
                on_selection_change=move |state| set_selection.set(state)
                extensions=TiptapExtension::all_enabled()
                attr:style="display: block; width: auto; height: auto; border: 1px solid; padding: 0.5em; white-space: pre-wrap;"
            />

            <div style="display: flex; flex-direction: row; gap: 0.5em; margin-top: 0.5em;">
                <div style="border: 1px solid; padding: 0.5em; min-width: 9em;">
                    <h2>"Selection"</h2>

                    { move || {
                        let selection = selection.get();

                        view! {
                            <table id="selection-state">
                                <thead>
                                    <tr>
                                        <th>"State"</th>
                                        <th>"Value"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td>"H1"</td>
                                        <td class="value" class:active=selection.h1>{ selection.h1 }</td>
                                    </tr>
                                    <tr>
                                        <td>"H2"</td>
                                        <td class="value" class:active=selection.h2>{ selection.h2 }</td>
                                    </tr>
                                    <tr>
                                        <td>"H3"</td>
                                        <td class="value" class:active=selection.h3>{ selection.h3 }</td>
                                    </tr>
                                    <tr>
                                        <td>"H4"</td>
                                        <td class="value" class:active=selection.h4>{ selection.h4 }</td>
                                    </tr>
                                    <tr>
                                        <td>"H5"</td>
                                        <td class="value" class:active=selection.h5>{ selection.h5 }</td>
                                    </tr>
                                    <tr>
                                        <td>"H6"</td>
                                        <td class="value" class:active=selection.h6>{ selection.h6 }</td>
                                    </tr>
                                    <tr>
                                        <td>"Paragraph"</td>
                                        <td class="value" class:active=selection.paragraph>{ selection.paragraph }</td>
                                    </tr>
                                    <tr>
                                        <td>"Bold"</td>
                                        <td class="value" class:active=selection.bold>{ selection.bold }</td>
                                    </tr>
                                    <tr>
                                        <td>"Italic"</td>
                                        <td class="value" class:active=selection.italic>{ selection.italic }</td>
                                    </tr>
                                    <tr>
                                        <td>"Strike"</td>
                                        <td class="value" class:active=selection.strike>{ selection.strike }</td>
                                    </tr>
                                    <tr>
                                        <td>"Blockquote"</td>
                                        <td class="value" class:active=selection.blockquote>{ selection.blockquote }</td>
                                    </tr>
                                    <tr>
                                        <td>"Highlight"</td>
                                        <td class="value" class:active=selection.highlight>{ selection.highlight }</td>
                                    </tr>
                                    <tr>
                                        <td>"Bullet List"</td>
                                        <td class="value" class:active=selection.bullet_list>{ selection.bullet_list }</td>
                                    </tr>
                                    <tr>
                                        <td>"Ordered List"</td>
                                        <td class="value" class:active=selection.ordered_list>{ selection.ordered_list }</td>
                                    </tr>
                                    <tr>
                                        <td>"Align left"</td>
                                        <td class="value" class:active=selection.align_left>{ selection.align_left }</td>
                                    </tr>
                                    <tr>
                                        <td>"Align center"</td>
                                        <td class="value" class:active=selection.align_center>{ selection.align_center }</td>
                                    </tr>
                                    <tr>
                                        <td>"Align right"</td>
                                        <td class="value" class:active=selection.align_right>{ selection.align_right }</td>
                                    </tr>
                                    <tr>
                                        <td>"Align justify"</td>
                                        <td class="value" class:active=selection.align_justify>{ selection.align_justify }</td>
                                    </tr>
                                    <tr>
                                        <td>"Link"</td>
                                        <td class="value" class:active=selection.link>{ selection.link }</td>
                                    </tr>
                                    <tr>
                                        <td>"YouTube"</td>
                                        <td class="value" class:active=selection.youtube>{ selection.youtube }</td>
                                    </tr>
                                </tbody>
                            </table>
                        }
                    } }
                </div>

                <div style="display: flex; flex-direction: column; flex-grow: 1;">
                    <div style="border: 1px solid; padding: 0.5em;">
                        <h2>"HTML content"</h2>

                        <pre id="html-content" style="margin: 0;
                                                      max-height: 35em;
                                                      overflow: auto;
                                                      white-space: break-spaces;">
                            {move || html_output.get()}
                        </pre>
                    </div>

                    <div style="border: 1px solid; padding: 0.5em; min-width: 20em;">
                        <h2>"JSON content"</h2>

                        <pre id="json-content" style="margin: 0;
                                                      max-height: 35em;
                                                      overflow: auto;
                                                      white-space: break-spaces;">
                            {move || json_output.get()}
                        </pre>
                    </div>
                </div>
            </div>
        </section>
    }
}

fn initial_html_content() -> TiptapContent {
    TiptapContent::html(
        r#"<h1>This is a simple <em><s>paragraph</s></em> ... <strong>H1</strong>!</h1><p style="text-align: center"><strong>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, <mark>sed diam nonumy</mark> eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.</strong></p><p style="text-align: justify">Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.</p>"#,
    )
}

fn replacement_html_content() -> TiptapContent {
    TiptapContent::html(
        r#"<h2>Programmatic replacement</h2><p>This content replaced the live document through <code>TiptapEditorHandle::set_content</code>.</p>"#,
    )
}

fn replace_editor_content(
    editor: &TiptapEditor,
    set_html_output: WriteSignal<String>,
    set_json_output: WriteSignal<String>,
) {
    match editor.set_content(replacement_html_content()) {
        Ok(()) => {}
        Err(err) => {
            set_html_output.set(format!("Error replacing content: {err}"));
            set_json_output.set(format!("Error replacing content: {err}"));
        }
    }
}

fn sync_editor_outputs(
    editor: &TiptapEditor,
    set_html_output: WriteSignal<String>,
    set_json_output: WriteSignal<String>,
) {
    set_html_output.set(
        editor
            .get_html()
            .unwrap_or_else(|err| format!("Error reading HTML content: {err}")),
    );
    set_json_output.set(
        editor
            .get_json()
            .map(|content| serde_json::to_string_pretty(&content).unwrap())
            .unwrap_or_else(|err| format!("Error reading JSON content: {err}")),
    );
}
