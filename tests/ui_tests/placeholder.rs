use crate::Context;
use crate::ui_tests::{
    click_button, click_css, goto, wait_for_attribute, wait_for_attribute_absent, wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Confirms the Tiptap placeholder extension is reactive: the
/// `data-placeholder` attribute is present on the empty paragraph, disappears
/// once content is typed, and reappears after the document is cleared.
pub struct RendersPlaceholderText;

const PLACEHOLDER_PARAGRAPH: &str = "#placeholder-editor .ProseMirror p";
const PLACEHOLDER_TEXT: &str = "Type something here...";

#[async_trait]
impl BrowserTest<Context> for RendersPlaceholderText {
    fn name(&self) -> Cow<'_, str> {
        "renders_placeholder_text".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/placeholder", context.base_url)).await?;

        wait_for_visible(driver, "#placeholder-editor .ProseMirror").await?;
        wait_for_attribute(
            driver,
            PLACEHOLDER_PARAGRAPH,
            "data-placeholder",
            PLACEHOLDER_TEXT,
        )
        .await?;

        let editor = wait_for_visible(driver, "#placeholder-editor .ProseMirror").await?;
        editor.click().await?;
        editor.send_keys("x").await?;
        wait_for_attribute_absent(
            driver,
            PLACEHOLDER_PARAGRAPH,
            "data-placeholder",
            PLACEHOLDER_TEXT,
        )
        .await?;

        click_css(driver, "#placeholder-editor .ProseMirror").await?;
        click_button(driver, "Clear").await?;
        wait_for_attribute(
            driver,
            PLACEHOLDER_PARAGRAPH,
            "data-placeholder",
            PLACEHOLDER_TEXT,
        )
        .await?;

        Ok(())
    }
}
