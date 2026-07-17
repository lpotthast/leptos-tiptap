use crate::Context;
use crate::ui_tests::{
    click_button, goto, wait_for_absent, wait_for_text_equals, wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Keeps one handle in the parent while its editor is unmounted and remounted. The fixture records
/// the public command state immediately after the replacement session claims the handle, before its
/// synchronous JS create reaches ready.
pub struct RemountsDestroyedHandleThroughNotReady;

#[async_trait]
impl BrowserTest<Context> for RemountsDestroyedHandleThroughNotReady {
    fn name(&self) -> Cow<'_, str> {
        "remounts_destroyed_handle_through_not_ready".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/remount-handle", context.base_url)).await?;

        wait_for_text_equals(driver, "#remount-claim-state", "NotReady").await?;
        wait_for_visible(driver, "#remount-editor .ProseMirror").await?;
        wait_for_text_equals(driver, "#remount-ready-state", "true").await?;

        click_button(driver, "Unmount editor").await?;
        wait_for_absent(driver, "#remount-editor .ProseMirror").await?;
        click_button(driver, "Inspect remount handle").await?;
        wait_for_text_equals(driver, "#remount-inspected-state", "Destroyed").await?;

        click_button(driver, "Remount editor").await?;
        wait_for_text_equals(driver, "#remount-claim-state", "NotReady").await?;
        wait_for_visible(driver, "#remount-editor .ProseMirror").await?;
        wait_for_text_equals(driver, "#remount-ready-state", "true").await?;
        click_button(driver, "Inspect remount handle").await?;
        wait_for_text_equals(driver, "#remount-inspected-state", "Ready").await?;

        Ok(())
    }
}

/// Starts with a synchronous extension-validation failure, then retries the same logical editor
/// with the same handle and a valid extension set.
pub struct RetriesFailedHandleThroughNotReady;

#[async_trait]
impl BrowserTest<Context> for RetriesFailedHandleThroughNotReady {
    fn name(&self) -> Cow<'_, str> {
        "retries_failed_handle_through_not_ready".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/retry-handle", context.base_url)).await?;

        wait_for_text_equals(driver, "#retry-failed-state", "CreateFailed").await?;
        wait_for_absent(driver, "#retry-editor .ProseMirror").await?;

        click_button(driver, "Retry editor").await?;
        wait_for_text_equals(driver, "#retry-claim-state", "NotReady").await?;
        wait_for_visible(driver, "#retry-editor .ProseMirror").await?;
        wait_for_text_equals(driver, "#retry-ready-state", "true").await?;
        click_button(driver, "Inspect retry handle").await?;
        wait_for_text_equals(driver, "#retry-inspected-state", "Ready").await?;

        Ok(())
    }
}
