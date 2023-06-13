use leptos::*;

mod wasm_tracing_layer;
use leptos_tiptap::tiptap_instance::{Selection, TiptapInstance, TiptapInstanceMsg};

fn main() {
    console_error_panic_hook::set_once();

    // Initialize a tracing subscriber logging to the browser console.
    wasm_tracing_layer::set_as_global_default_with_config(
        wasm_tracing_layer::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );

    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (msg, set_msg) = create_signal(cx, TiptapInstanceMsg::Noop);
    let (value, set_value) = create_signal(cx, r#"<h1>This is a simple <em><s>paragraph</s></em> ... <strong>H1</strong>!</h1><p style="text-align: center"><strong>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, <mark>sed diam nonumy</mark> eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.</strong></p><p style="text-align: justify">Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.</p>"#.to_owned());

    let (selection, set_selection) = create_signal(cx, Selection::default());

    view! {cx,
        <h2>"Tiptap instance with Controls"</h2>

        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H1)>"H1"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H2)>"H2"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H3)>"H3"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H4)>"H4"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H5)>"H5"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H6)>"H6"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Paragraph)>"Paragraph"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Bold)>"Bold"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Italic)>"Italic"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Strike)>"Strike"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Blockquote)>"Blockquote"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Highlight)>"Highlight"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::AlignLeft)>"AlignLeft"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::AlignCenter)>"AlignCenter"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::AlignRight)>"AlignRight"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::AlignJustify)>"AlignJustify"</button>

        <TiptapInstance
            id="id"
            msg=msg
            disabled=false
            value=value
            set_value=move |v| set_value.set(v.content)
            on_selection_change=move |state| set_selection.set(state)
            style="display: block; width: 100%; height: auto; border: 1px solid;"
        />

        <hr/>

        <h2>"Selection state"</h2>

        { move || {
            let selection = selection.get();

            view! {cx,
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
                            <td class:active=selection.state.h1>{ selection.state.h1 }</td>
                        </tr>
                        <tr>
                            <td>"H2"</td>
                            <td class:active=selection.state.h2>{ selection.state.h2 }</td>
                        </tr>
                        <tr>
                            <td>"H3"</td>
                            <td class:active=selection.state.h3>{ selection.state.h3 }</td>
                        </tr>
                        <tr>
                            <td>"H4"</td>
                            <td class:active=selection.state.h4>{ selection.state.h4 }</td>
                        </tr>
                        <tr>
                            <td>"H5"</td>
                            <td class:active=selection.state.h5>{ selection.state.h5 }</td>
                        </tr>
                        <tr>
                            <td>"H6"</td>
                            <td class:active=selection.state.h6>{ selection.state.h6 }</td>
                        </tr>
                        <tr>
                            <td>"Paragraph"</td>
                            <td class:active=selection.state.paragraph>{ selection.state.paragraph }</td>
                        </tr>
                        <tr>
                            <td>"Bold"</td>
                            <td class:active=selection.state.bold>{ selection.state.bold }</td>
                        </tr>
                        <tr>
                            <td>"Italic"</td>
                            <td class:active=selection.state.italic>{ selection.state.italic }</td>
                        </tr>
                        <tr>
                            <td>"Strike"</td>
                            <td class:active=selection.state.strike>{ selection.state.strike }</td>
                        </tr>
                        <tr>
                            <td>"Blockquote"</td>
                            <td class:active=selection.state.blockquote>{ selection.state.blockquote }</td>
                        </tr>
                        <tr>
                            <td>"Highlight"</td>
                            <td class:active=selection.state.highlight>{ selection.state.highlight }</td>
                        </tr>
                        <tr>
                            <td>"Align left"</td>
                            <td class:active=selection.state.align_left>{ selection.state.align_left }</td>
                        </tr>
                        <tr>
                            <td>"Align center"</td>
                            <td class:active=selection.state.align_center>{ selection.state.align_center }</td>
                        </tr>
                        <tr>
                            <td>"Align right"</td>
                            <td class:active=selection.state.align_right>{ selection.state.align_right }</td>
                        </tr>
                        <tr>
                            <td>"Align justify"</td>
                            <td class:active=selection.state.align_justify>{ selection.state.align_justify }</td>
                        </tr>
                    </tbody>
                </table>
            }
        } }

        <hr/>

        <h2>"HTML content"</h2>

        <div>
            { move || value.get() }
        </div>
    }
}
