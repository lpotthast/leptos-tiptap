use crate::ui_tests::{
    EDITOR_SELECTOR, HTML_CONTENT_SELECTOR, click_button_starting_with, goto, wait_for_attribute,
    wait_for_text_contains, wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;
use crate::Context;

pub struct ReEnablesEditorAfterDisabling;

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

        click_button_starting_with(driver, "Disabled:").await?;
        wait_for_attribute(driver, EDITOR_SELECTOR, "contenteditable", "true").await?;

        let editor = wait_for_visible(driver, EDITOR_SELECTOR).await?;
        editor.click().await?;
        editor.send_keys(" re-enabled").await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "re-enabled").await?;

        Ok(())
    }
}
