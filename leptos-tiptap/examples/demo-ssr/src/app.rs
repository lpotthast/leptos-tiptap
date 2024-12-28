use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_tiptap::{
    TiptapContent, TiptapInstance, TiptapInstanceMsg, TiptapLinkResource, TiptapSelectionState,
    TiptapYoutubeVideoResource,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script type_="module" src="/js/tiptap-bundle.min.js"/>
                <script type_="module" src="/js/tiptap.js"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/demo-ssr.css"/>

        // sets the document title
        <Title text="Tiptap demo ssr"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=Demo/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Demo() -> impl IntoView {
    let (msg, set_msg) = signal(TiptapInstanceMsg::Noop);
    let (value, set_value) = signal(r#"<h1>This is a simple <em><s>paragraph</s></em> ... <strong>H1</strong>!</h1><p style="text-align: center"><strong>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, <mark>sed diam nonumy</mark> eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.</strong></p><p style="text-align: justify">Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.</p>"#.to_owned());
    let (selection, set_selection) = signal(TiptapSelectionState::default());
    let (disabled, set_disabled) = signal(false);

    view! {
        <h2>"Tiptap instance"</h2>

        <button on:click=move |_| set_disabled.set(!disabled.get())>"Disabled: " { move || disabled.get() }</button>
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
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::SetLink(TiptapLinkResource{href: "https://www.google.com/".to_string(), target: "_blank".to_string(), rel: "alternate".to_string()}))>"Add link"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::ToggleLink(TiptapLinkResource{href: "https://www.google.com/".to_string(), target: "_blank".to_string(), rel: "alternate".to_string()}))>"Toggle link"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::UnsetLink())>"Unset link"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::SetYoutubeVideo(TiptapYoutubeVideoResource{src: "https://www.youtube.com/embed/dQw4w9WgXcQ?si=6LwJzVo1t8hpLywC".to_string(), start: "0".to_string(), width:"640".to_string(), height: "480".to_string()}))>"Toggle youtube video"</button>

        <TiptapInstance
            id="id"
            msg=msg
            disabled=disabled
            value=value
            set_value=Callback::new(move |v| set_value.set(match v {
                TiptapContent::Html(content) => content,
                TiptapContent::Json(content) => content,
            }))
            on_selection_change=Callback::new(move |state: TiptapSelectionState| {
                set_selection.set(state);
            })
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
                            </tbody>
                        </table>
                    }
                } }
            </div>

            <div style="border: 1px solid; padding: 0.5em;">
                <h2>"HTML content"</h2>

                <div>
                    { move || value.get() }
                </div>
            </div>
        </div>
    }
}
