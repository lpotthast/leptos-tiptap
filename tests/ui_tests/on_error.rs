use crate::Context;
use crate::ui_tests::{goto, wait_for_absent, wait_for_text_contains};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Mounts an editor with an invalid extension subset (`text_align` without its
/// required `heading`/`paragraph` extensions). Validation fails synchronously
/// and the resulting `TiptapEditorReport` is routed through the `on_error`
/// callback, which the fixture renders into `#error-message`. Also asserts the
/// error names the missing dependencies (`heading` and `paragraph`) and that
/// the editor itself did not mount.
pub struct EmitsErrorReportToOnErrorCallback;

#[async_trait]
impl BrowserTest<Context> for EmitsErrorReportToOnErrorCallback {
    fn name(&self) -> Cow<'_, str> {
        "emits_error_report_to_on_error_callback".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/on-error", context.base_url)).await?;

        wait_for_text_contains(driver, "#error-message", "missing required extension").await?;
        wait_for_text_contains(driver, "#error-message", "heading").await?;
        wait_for_text_contains(driver, "#error-message", "paragraph").await?;
        wait_for_absent(driver, "#error-editor .ProseMirror").await?;

        Ok(())
    }
}
