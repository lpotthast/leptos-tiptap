use crate::Context;
use crate::ui_tests::{
    EDITOR_SELECTOR, HTML_CONTENT_SELECTOR, click_button_starting_with, goto, wait_for_attribute,
    wait_for_text_contains, wait_for_text_not_contains, wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Toggles the editor from enabled to disabled and back, asserting both the
/// `contenteditable` attribute flips and that the disabled editor actually
/// rejects keystrokes (the attribute flipping is not enough to prove the
/// editor is functionally disabled). After re-enabling, asserts the editor
/// accepts input again.
pub struct ReEnablesEditorAfterDisabling;

const DISABLED_SENTINEL: &str = "should-not-appear-while-disabled";

#[async_trait]
impl BrowserTest<Context> for ReEnablesEditorAfterDisabling {
    fn name(&self) -> Cow<'_, str> {
        "re_enables_editor_after_disabling".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &context.base_url).await?;

        wait_for_visible(driver, EDITOR_SELECTOR).await?;
        wait_for_attribute(driver, EDITOR_SELECTOR, "contenteditable", "true").await?;

        click_button_starting_with(driver, "Disabled:").await?;
        wait_for_attribute(driver, EDITOR_SELECTOR, "contenteditable", "false").await?;

        let editor = wait_for_visible(driver, EDITOR_SELECTOR).await?;
        let _ = editor.click().await;
        let _ = editor.send_keys(DISABLED_SENTINEL).await;
        wait_for_text_not_contains(driver, HTML_CONTENT_SELECTOR, DISABLED_SENTINEL).await?;

        click_button_starting_with(driver, "Disabled:").await?;
        wait_for_attribute(driver, EDITOR_SELECTOR, "contenteditable", "true").await?;
        wait_for_text_not_contains(driver, HTML_CONTENT_SELECTOR, DISABLED_SENTINEL).await?;

        let editor = wait_for_visible(driver, EDITOR_SELECTOR).await?;
        editor.click().await?;
        editor.send_keys(" re-enabled").await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "re-enabled").await?;

        Ok(())
    }
}
