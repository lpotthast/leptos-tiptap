use crate::Context;
use crate::ui_tests::{
    click_button, goto, wait_for_absent, wait_for_text_contains, wait_for_text_equals,
    wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Mounts a second component only after the first editor is live, then verifies that creation
/// crosses the JS bridge, reports the duplicate ID, and leaves the original editor operational.
pub struct RejectsDuplicateLiveEditorId;

#[async_trait]
impl BrowserTest<Context> for RejectsDuplicateLiveEditorId {
    fn name(&self) -> Cow<'_, str> {
        "rejects_duplicate_live_editor_id".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/duplicate-editor-id", context.base_url)).await?;

        wait_for_text_equals(driver, "#duplicate-original-ready", "true").await?;
        wait_for_visible(driver, "#duplicate-original .ProseMirror").await?;

        click_button(driver, "Mount duplicate editor").await?;

        wait_for_text_contains(driver, "#duplicate-error", "another live editor").await?;
        wait_for_text_equals(driver, "#duplicate-rejected-state", "CreateFailed").await?;
        wait_for_absent(driver, "#duplicate-rejected .ProseMirror").await?;

        click_button(driver, "H1 original").await?;
        wait_for_visible(driver, "#duplicate-original .ProseMirror h1").await?;

        Ok(())
    }
}
