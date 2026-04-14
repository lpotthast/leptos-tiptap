use leptos_browser_test::{BrowserTest, Report, WebDriver, async_trait};
use std::borrow::Cow;
use thirtyfour::{By, WebElement, prelude::ElementQueryable};

const EDITOR_SELECTOR: &str = "#id .ProseMirror";
const HTML_CONTENT_SELECTOR: &str = "#html-content";
const JSON_CONTENT_SELECTOR: &str = "#json-content";

pub struct BuffersToolbarCommandBeforeEditorReady;

#[async_trait]
impl BrowserTest<str> for BuffersToolbarCommandBeforeEditorReady {
    fn name(&self) -> Cow<'_, str> {
        "buffers_toolbar_command_before_editor_ready".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        goto(driver, base_url).await?;

        click_button(driver, "H2").await?;

        wait_for_visible(driver, EDITOR_SELECTOR).await?;
        wait_for_text_contains(driver, HTML_CONTENT_SELECTOR, "<h2").await?;
        wait_for_text_contains(driver, JSON_CONTENT_SELECTOR, "\"level\": 2").await?;

        Ok(())
    }
}

pub struct HydratesAndRoundTripsContent;

#[async_trait]
impl BrowserTest<str> for HydratesAndRoundTripsContent {
    fn name(&self) -> Cow<'_, str> {
        "hydrates_and_round_trips_content".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        goto(driver, base_url).await?;

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

pub struct ReEnablesEditorAfterDisabling;

#[async_trait]
impl BrowserTest<str> for ReEnablesEditorAfterDisabling {
    fn name(&self) -> Cow<'_, str> {
        "re_enables_editor_after_disabling".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        goto(driver, base_url).await?;

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

#[tracing::instrument(
    name = "browser_test_step",
    skip_all,
    fields(helper = "goto", base_url = %base_url),
)]
async fn goto(driver: &WebDriver, base_url: &str) -> Result<(), Report> {
    driver.goto(base_url).await?;
    Ok(())
}

#[tracing::instrument(
    name = "browser_test_step",
    skip_all,
    fields(helper = "click_button", text = %text),
)]
async fn click_button(driver: &WebDriver, text: &str) -> Result<(), Report> {
    let selector = format!("//button[normalize-space(.)={text:?}]");
    let button = wait_for_clickable(driver, By::XPath(&selector)).await?;
    scroll_into_view(driver, &button).await?;
    click_with_script(driver, &button).await
}

#[tracing::instrument(
    name = "browser_test_step",
    skip_all,
    fields(helper = "click_button_starting_with", text = %text),
)]
async fn click_button_starting_with(driver: &WebDriver, text: &str) -> Result<(), Report> {
    let selector = format!("//button[starts-with(normalize-space(.), {text:?})]");
    let button = wait_for_clickable(driver, By::XPath(&selector)).await?;
    scroll_into_view(driver, &button).await?;
    click_with_script(driver, &button).await
}

#[tracing::instrument(
    name = "browser_test_step",
    skip_all,
    fields(helper = "click_css", selector = %selector),
)]
async fn click_css(driver: &WebDriver, selector: &str) -> Result<(), Report> {
    let element = wait_for_clickable(driver, By::Css(selector)).await?;
    scroll_into_view(driver, &element).await?;
    element.click().await?;
    Ok(())
}

async fn scroll_into_view(driver: &WebDriver, element: &WebElement) -> Result<(), Report> {
    let element_json = element.to_json()?;
    driver
        .execute(
            "arguments[0].scrollIntoView({ block: 'center', inline: 'center' });",
            vec![element_json],
        )
        .await?;
    Ok(())
}

async fn click_with_script(driver: &WebDriver, element: &WebElement) -> Result<(), Report> {
    let element_json = element.to_json()?;
    driver
        .execute("arguments[0].click();", vec![element_json])
        .await?;
    Ok(())
}

async fn wait_for_clickable(driver: &WebDriver, by: By) -> Result<WebElement, Report> {
    let element = driver.query(by).and_clickable().first().await?;
    Ok(element)
}

#[tracing::instrument(
    name = "browser_test_step",
    skip_all,
    fields(helper = "wait_for_visible", selector = %selector),
)]
async fn wait_for_visible(driver: &WebDriver, selector: &str) -> Result<WebElement, Report> {
    let element = driver
        .query(By::Css(selector))
        .and_displayed()
        .first()
        .await?;
    Ok(element)
}

#[tracing::instrument(
    name = "browser_test_step",
    skip_all,
    fields(helper = "wait_for_single", selector = %selector),
)]
async fn wait_for_single(driver: &WebDriver, selector: &str) -> Result<(), Report> {
    driver.query(By::Css(selector)).single().await?;
    Ok(())
}

#[tracing::instrument(
    name = "browser_test_step",
    skip_all,
    fields(helper = "wait_for_text_contains", selector = %selector, expected = %expected),
)]
async fn wait_for_text_contains(
    driver: &WebDriver,
    selector: &str,
    expected: &str,
) -> Result<(), Report> {
    let expected = expected.to_owned();
    driver
        .query(By::Css(selector))
        .with_text(move |text: &str| text.contains(&expected))
        .first()
        .await?;
    Ok(())
}

#[tracing::instrument(
    name = "browser_test_step",
    skip_all,
    fields(
        helper = "wait_for_attribute",
        selector = %selector,
        attribute = %name,
        expected = %expected,
    ),
)]
async fn wait_for_attribute(
    driver: &WebDriver,
    selector: &str,
    name: &str,
    expected: &str,
) -> Result<(), Report> {
    driver
        .query(By::Css(selector))
        .with_attribute(name.to_owned(), expected.to_owned())
        .first()
        .await?;
    Ok(())
}
