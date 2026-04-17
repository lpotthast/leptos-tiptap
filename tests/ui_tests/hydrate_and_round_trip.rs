use crate::Context;
use crate::ui_tests::{
    EDITOR_SELECTOR, HTML_CONTENT_SELECTOR, JSON_CONTENT_SELECTOR, click_button, click_css,
    wait_for_single, wait_for_text_contains, wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

pub struct HydratesAndRoundTripsContent;

#[async_trait]
impl BrowserTest<Context> for HydratesAndRoundTripsContent {
    fn name(&self) -> Cow<'_, str> {
        "hydrates_and_round_trips_content".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        crate::ui_tests::goto(driver, &context.base_url).await?;

        wait_for_visible(driver, EDITOR_SELECTOR).await?;
        wait_for_single(driver, ".ProseMirror").await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "<h1").await?;
        wait_for_text_contains(driver, JSON_CONTENT_SELECTOR, "\"type\": \"doc\"").await?;

        click_css(driver, EDITOR_SELECTOR).await?;
        click_button(driver, "H2").await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "<h2").await?;
        wait_for_text_contains(driver, JSON_CONTENT_SELECTOR, "\"level\": 2").await?;

        let editor = wait_for_visible(driver, EDITOR_SELECTOR).await?;
        editor.click().await?;
        editor.send_keys(" updated").await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "updated").await?;
        wait_for_text_contains(driver, JSON_CONTENT_SELECTOR, "updated").await?;

        Ok(())
    }
}
