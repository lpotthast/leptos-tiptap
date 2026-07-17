use crate::Context;
use crate::ui_tests::{
    click_button, click_css, goto, wait_for_absent, wait_for_text_contains, wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Drives two editors mounted on the same page. Asserts both initialize independently and that a
/// command dispatched against one handle does not disturb the other (neither by leaking the new
/// heading into the sibling editor nor by clobbering the first command when the second is
/// dispatched).
pub struct MountsTwoEditorsIndependently;

#[async_trait]
impl BrowserTest<Context> for MountsTwoEditorsIndependently {
    fn name(&self) -> Cow<'_, str> {
        "mounts-two-editors-independently".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/multi-editor", context.base_url)).await?;

        wait_for_visible(driver, "#editor-a .ProseMirror").await?;
        wait_for_visible(driver, "#editor-b .ProseMirror").await?;
        wait_for_text_contains(driver, "#editor-a .ProseMirror", "Editor A.").await?;
        wait_for_text_contains(driver, "#editor-b .ProseMirror", "Editor B.").await?;

        click_css(driver, "#editor-a .ProseMirror").await?;
        click_button(driver, "H1 A").await?;

        wait_for_visible(driver, "#editor-a .ProseMirror h1").await?;
        wait_for_absent(driver, "#editor-b .ProseMirror h1").await?;
        wait_for_absent(driver, "#editor-b .ProseMirror h2").await?;

        click_css(driver, "#editor-b .ProseMirror").await?;
        click_button(driver, "H2 B").await?;

        wait_for_visible(driver, "#editor-b .ProseMirror h2").await?;
        wait_for_visible(driver, "#editor-a .ProseMirror h1").await?;
        wait_for_absent(driver, "#editor-a .ProseMirror h2").await?;
        wait_for_absent(driver, "#editor-b .ProseMirror h1").await?;

        Ok(())
    }
}
