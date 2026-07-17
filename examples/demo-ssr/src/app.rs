use demo_app::test_fixtures::{
    DuplicateEditorIdFixture, ExtensionSubsetFixture, MultiEditorFixture, OnChangeCountingFixture,
    OnErrorFixture, PlaceholderFixture, RemountHandleFixture, RetryHandleFixture,
};
use demo_app::DemoApp;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
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
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/demo-ssr.css"/>
        <Title text="Tiptap demo ssr"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=DemoApp/>
                    <Route path=StaticSegment("multi-editor") view=MultiEditorFixture/>
                    <Route path=StaticSegment("duplicate-editor-id") view=DuplicateEditorIdFixture/>
                    <Route path=StaticSegment("placeholder") view=PlaceholderFixture/>
                    <Route path=StaticSegment("extension-subset") view=ExtensionSubsetFixture/>
                    <Route path=StaticSegment("on-error") view=OnErrorFixture/>
                    <Route path=StaticSegment("on-change-counting") view=OnChangeCountingFixture/>
                    <Route path=StaticSegment("remount-handle") view=RemountHandleFixture/>
                    <Route path=StaticSegment("retry-handle") view=RetryHandleFixture/>
                </Routes>
            </main>
        </Router>
    }
}
