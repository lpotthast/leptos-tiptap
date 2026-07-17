use crate::Context;
use crate::ui_tests::{
    EDITOR_SELECTOR, HTML_CONTENT_SELECTOR, JSON_CONTENT_SELECTOR, click_button, goto,
    wait_for_text_contains, wait_for_text_not_contains, wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Drives the demo "Replace content" button so the editor's live document is
/// replaced through `TiptapEditorHandle::set_content`. Confirms the
/// `on_change` callback fires, that the new content is observable through both
/// the HTML and JSON readback panes, and that the original content is gone
/// (i.e. the operation replaced, did not append).
pub struct ReplacesLiveContent;

#[async_trait]
impl BrowserTest<Context> for ReplacesLiveContent {
    fn name(&self) -> Cow<'_, str> {
        "replaces_live_content".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &context.base_url).await?;

        wait_for_visible(driver, EDITOR_SELECTOR).await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "<h1").await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "Lorem ipsum").await?;

        click_button(driver, "Replace content").await?;

        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "Programmatic replacement").await?;
        wait_for_text_contains(driver, JSON_CONTENT_SELECTOR, "Programmatic replacement").await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "<h2").await?;
        wait_for_text_not_contains(driver, HTML_CONTENT_SELECTOR, "<h1").await?;
        wait_for_text_not_contains(driver, HTML_CONTENT_SELECTOR, "Lorem ipsum").await?;
        wait_for_text_contains(driver, JSON_CONTENT_SELECTOR, "\"level\": 2").await?;

        Ok(())
    }
}
