use crate::Context;
use crate::ui_tests::{
    click_button, click_css, goto, wait_for_absent, wait_for_text_contains, wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Confirms an editor mounted with an explicit per-instance extension subset
/// (document/paragraph/text/heading) initializes successfully, dispatches a
/// command from that subset (toggle heading H3), and rejects a command for an
/// extension that was deliberately excluded (toggle bold).
pub struct ActivatesExtensionSubset;

#[async_trait]
impl BrowserTest<Context> for ActivatesExtensionSubset {
    fn name(&self) -> Cow<'_, str> {
        "activates_extension_subset".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/extension-subset", context.base_url)).await?;

        wait_for_visible(driver, "#subset-editor .ProseMirror").await?;

        click_css(driver, "#subset-editor .ProseMirror").await?;
        click_button(driver, "H3").await?;
        wait_for_visible(driver, "#subset-editor .ProseMirror h3").await?;

        click_css(driver, "#subset-editor .ProseMirror").await?;
        click_button(driver, "Bold").await?;
        wait_for_absent(driver, "#subset-editor .ProseMirror strong").await?;
        wait_for_text_contains(driver, "#subset-error", "bold").await?;

        Ok(())
    }
}
