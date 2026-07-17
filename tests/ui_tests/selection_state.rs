use crate::Context;
use crate::ui_tests::{EDITOR_SELECTOR, click_button, click_css, goto, wait_for_visible};
use browser_test::thirtyfour::prelude::ElementQueryable;
use browser_test::thirtyfour::{By, Key, WebDriver};
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Drives the main `DemoApp` and exercises the `on_selection_change` callback:
/// selects all editor content, applies bold via the toolbar button, then
/// asserts the Bold row in `#selection-state` reports the bold flag as active
/// (via the `.active` class on its value cell).
pub struct ObservesSelectionStateBoldFlag;

#[async_trait]
impl BrowserTest<Context> for ObservesSelectionStateBoldFlag {
    fn name(&self) -> Cow<'_, str> {
        "observes_selection_state_bold_flag".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &context.base_url).await?;

        let editor = wait_for_visible(driver, EDITOR_SELECTOR).await?;
        click_css(driver, EDITOR_SELECTOR).await?;
        editor.send_keys(Key::Control + "a").await?;

        click_button(driver, "Bold").await?;

        driver
            .query(By::XPath(
                "//table[@id='selection-state']\
                 //tr[td[normalize-space()='Bold']]\
                 /td[contains(concat(' ', normalize-space(@class), ' '), ' active ')]",
            ))
            .first()
            .await?;

        Ok(())
    }
}
