//! Browser integration tests for the SSR demo editor lifecycle.
#![cfg(not(target_arch = "wasm32"))]

mod ui_tests;

use std::time::Duration;

use leptos_browser_test::{
    BrowserDriverOutputConfig, BrowserTest, BrowserTestRunner, ElementQueryWaitConfig,
    LeptosTestAppConfig, TimeoutConfiguration,
};
use rootcause::Report;
use rootcause::hooks::Hooks;
use rootcause::prelude::ResultExt;
use rootcause_backtrace::BacktraceCollector;
use rootcause_tracing::{RootcauseLayer, SpanCollector};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};
use ui_tests::test_editor::{
    BuffersToolbarCommandBeforeEditorReady, HydratesAndRoundTripsContent,
    ReEnablesEditorAfterDisabling,
};

const WEBDRIVER_SCRIPT_TIMEOUT: Duration = Duration::from_secs(5);
const WEBDRIVER_PAGE_LOAD_TIMEOUT: Duration = Duration::from_secs(5);
const WEBDRIVER_IMPLICIT_WAIT_TIMEOUT: Duration = Duration::from_secs(0);
const ELEMENT_QUERY_TIMEOUT: Duration = Duration::from_secs(5);
const ELEMENT_QUERY_INTERVAL: Duration = Duration::from_millis(500);

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    // Set up tracing with RootcauseLayer.
    let subscriber = Registry::default().with(RootcauseLayer).with(
        tracing_subscriber::fmt::layer()
            .with_test_writer()
            .with_filter(LevelFilter::INFO),
    );
    tracing::subscriber::set_global_default(subscriber)
        .context("Setting global tracing subscriber")?;

    // Capture spans and backtraces for all errors.
    Hooks::new()
        .report_creation_hook(SpanCollector {
            capture_span_for_reports_with_children: false,
        })
        .report_creation_hook(BacktraceCollector {
            capture_backtrace_for_reports_with_children: false,
            ..BacktraceCollector::new_from_env()
        })
        .install()
        .context("Installing rootcause hooks")?;

    let app = LeptosTestAppConfig::new("examples/demo-ssr")
        .with_app_name("leptos-tiptap demo ssr")
        .start()
        .await
        .context("Starting examples/demo-ssr app")?;

    let tests: Vec<Box<dyn BrowserTest<str>>> = vec![
        Box::new(BuffersToolbarCommandBeforeEditorReady),
        Box::new(HydratesAndRoundTripsContent),
        Box::new(ReEnablesEditorAfterDisabling),
    ];

    BrowserTestRunner::new()
        .with_default_visible_env()
        .with_default_pause_env()
        .with_webdriver_timeouts(TimeoutConfiguration::new(
            Some(WEBDRIVER_SCRIPT_TIMEOUT),
            Some(WEBDRIVER_PAGE_LOAD_TIMEOUT),
            Some(WEBDRIVER_IMPLICIT_WAIT_TIMEOUT),
        ))
        .with_element_query_wait(ElementQueryWaitConfig::new(
            ELEMENT_QUERY_TIMEOUT,
            ELEMENT_QUERY_INTERVAL,
        ))
        .with_browser_driver_output(BrowserDriverOutputConfig { tail_lines: 100 })
        .with_hint(format!("Leptos test app is running at {}", app.base_url()))
        .run(app.base_url(), tests)
        .await
        .context("Running tests")?;

    Ok(())
}
