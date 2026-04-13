//! Browser integration tests for the SSR demo editor lifecycle.
#![cfg(not(target_arch = "wasm32"))]

mod ui_tests;

use leptos_browser_test::{BrowserTest, BrowserTestRunner, LeptosTestAppConfig};
use ui_tests::test_editor::{
    BuffersToolbarCommandBeforeEditorReady, HydratesAndRoundTripsContent,
    ReEnablesEditorAfterDisabling,
};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), String> {
    tracing_subscriber::fmt().with_test_writer().init();

    let app = LeptosTestAppConfig::new("examples/demo-ssr")
        .with_app_name("leptos-tiptap demo ssr")
        .start()
        .await
        .map_err(|err| err.to_string())?;

    let tests: Vec<Box<dyn BrowserTest<str>>> = vec![
        Box::new(BuffersToolbarCommandBeforeEditorReady),
        Box::new(HydratesAndRoundTripsContent),
        Box::new(ReEnablesEditorAfterDisabling),
    ];

    BrowserTestRunner::new()
        .with_default_visible_env()
        .with_default_pause_env()
        .with_hint(format!("Leptos test app is running at {}", app.base_url()))
        .run(app.base_url(), tests)
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}
