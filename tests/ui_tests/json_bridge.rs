use crate::Context;
use crate::ui_tests::{
    click_button, goto, wait_for_text_contains, wait_for_text_equals, wait_for_text_not_contains,
    wait_for_visible,
};
use browser_test::thirtyfour::WebDriver;
use browser_test::{BrowserTest, async_trait};
use rootcause::Report;
use std::borrow::Cow;

/// Covers the request shapes that contain arbitrary JSON maps: editor creation,
/// document replacement, and generic Tiptap attributes.
pub struct PreservesJsonObjectsAcrossTheWasmBridge;

#[async_trait]
impl BrowserTest<Context> for PreservesJsonObjectsAcrossTheWasmBridge {
    fn name(&self) -> Cow<'_, str> {
        "preserves_json_objects_across_the_wasm_bridge".into()
    }

    async fn run(&self, driver: &WebDriver, context: &Context) -> Result<(), Report> {
        goto(driver, &format!("{}/json-bridge", context.base_url)).await?;

        wait_for_visible(driver, "#json-bridge-editor .ProseMirror").await?;
        wait_for_text_contains(driver, "#json-bridge-json", "\"type\": \"doc\"").await?;
        wait_for_text_contains(driver, "#json-bridge-json", "Initial JSON content.").await?;
        wait_for_text_equals(driver, "#json-bridge-status", "ok").await?;

        click_button(driver, "Set JSON").await?;

        wait_for_text_contains(driver, "#json-bridge-json", "Replacement JSON content.").await?;
        wait_for_text_not_contains(driver, "#json-bridge-json", "Initial JSON content.").await?;
        wait_for_text_equals(driver, "#json-bridge-status", "ok").await?;

        click_button(driver, "Set generic link attributes").await?;

        wait_for_visible(
            driver,
            "#json-bridge-editor .ProseMirror a[href='https://example.com/bridge'][target='_blank'][rel='noopener']",
        )
        .await?;
        wait_for_text_contains(driver, "#json-bridge-html", "https://example.com/bridge").await?;
        wait_for_text_equals(driver, "#json-bridge-status", "ok").await?;

        Ok(())
    }
}
