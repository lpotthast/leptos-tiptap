//! Browser integration tests for the SSR demo editor lifecycle.
#![cfg(not(target_arch = "wasm32"))]

mod ui_tests;

use std::num::NonZeroUsize;
use std::time::Duration;

use browser_test::{
    BrowserTestParallelism, BrowserTestRunner, BrowserTestVisibility, BrowserTests,
    BrowserTimeouts, DriverOutputConfig, ElementQueryWaitConfig, PauseConfig,
};
use leptos_browser_test::LeptosTestAppConfig;
use rootcause::Report;
use rootcause::hooks::Hooks;
use rootcause::prelude::ResultExt;
use rootcause_backtrace::BacktraceCollector;
use rootcause_tracing::{RootcauseLayer, SpanCollector};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};
use ui_tests::hydrate_and_round_trip::HydratesAndRoundTripsContent;
use ui_tests::re_enable_after_disable::ReEnablesEditorAfterDisabling;
use ui_tests::replace_content::ReplacesLiveContent;

#[derive(Debug)]
struct Context {
    base_url: String,
}

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

    let context = Context {
        base_url: app.base_url().to_owned(),
    };

    let tests = BrowserTests::new()
        .with(HydratesAndRoundTripsContent)
        .with(ReEnablesEditorAfterDisabling)
        .with(ReplacesLiveContent);

    BrowserTestRunner::new()
        .with_visibility(BrowserTestVisibility::from_env())
        .with_pause(PauseConfig::from_env())
        .with_test_parallelism(BrowserTestParallelism::Parallel(
            NonZeroUsize::new(4).expect("non-zero"),
        ))
        .with_timeouts(
            BrowserTimeouts::builder()
                .script_timeout(Duration::from_secs(5))
                .page_load_timeout(Duration::from_secs(5))
                .implicit_wait_timeout(Duration::from_secs(0))
                .build(),
        )
        .with_element_query_wait(ElementQueryWaitConfig::new(
            Duration::from_secs(5),
            Duration::from_millis(250),
        ))
        .with_driver_output(DriverOutputConfig::tail_lines(100))
        .with_hint(format!("Leptos test app is running at {}", app.base_url()))
        .run(&context, tests)
        .await
        .context("Running tests")?;

    Ok(())
}
