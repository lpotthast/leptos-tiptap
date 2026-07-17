use crate::Context;
use crate::ui_tests::{click_button, goto, wait_for_text_equals, wait_for_visible};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;
use std::time::Duration;

/// Mounts an editor whose `on_change` callback increments a counter rendered
/// into `#on-change-count`. Performs one programmatic `set_content` and
/// asserts the counter went up by exactly 1, guarding against both missed
/// callbacks (counter stays at 0) and double-firing (counter goes to 2 from
/// one user action).
pub struct FiresOnChangeExactlyOncePerReplace;

#[async_trait]
impl BrowserTest<Context> for FiresOnChangeExactlyOncePerReplace {
    fn name(&self) -> Cow<'_, str> {
        "fires_on_change_exactly_once_per_replace".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/on-change-counting", context.base_url)).await?;

        wait_for_visible(driver, "#on-change-counting-editor .ProseMirror").await?;
        wait_for_text_equals(driver, "#on-change-count", "0").await?;

        click_button(driver, "Replace").await?;

        wait_for_text_equals(driver, "#on-change-count", "1").await?;

        tokio::time::sleep(Duration::from_millis(500)).await;
        wait_for_text_equals(driver, "#on-change-count", "1").await?;

        Ok(())
    }
}
