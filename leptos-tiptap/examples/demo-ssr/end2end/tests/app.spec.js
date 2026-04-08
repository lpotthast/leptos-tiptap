const { test, expect } = require('@playwright/test');

const EXPECTED_EDITOR_ID = 'id';
const editorSelector = `#${EXPECTED_EDITOR_ID} .ProseMirror`;

async function delayEditorLookup(page, id, delayMs) {
  await page.addInitScript(
    ({ id, delayMs }) => {
      const originalGetElementById = Document.prototype.getElementById;
      const blockUntil = Date.now() + delayMs;

      Document.prototype.getElementById = function (requestedId) {
        if (requestedId === id && Date.now() < blockUntil) {
          return null;
        }

        return originalGetElementById.call(this, requestedId);
      };
    },
    { id, delayMs },
  );
}

test('buffers a toolbar command sent before the editor is ready', async ({ page }) => {
  await delayEditorLookup(page, EXPECTED_EDITOR_ID, 80);
  await page.goto('/');

  await page.getByRole('button', { name: 'H2' }).click();

  await expect(page.locator(editorSelector)).toBeVisible();
  await expect(page.locator('#html-content')).toContainText('<h2');
  await expect(page.locator('#json-content')).toContainText('"level":2');
});

test('hydrates and round-trips HTML and JSON content with a stable editor id', async ({ page }) => {
  await page.goto('/');

  await expect(page.locator(editorSelector)).toBeVisible();
  await expect(page.locator('.ProseMirror')).toHaveCount(1);
  await expect(page.locator('#html-content')).toContainText('<h1');
  await expect(page.locator('#json-content')).toContainText('"type":"doc"');

  await page.locator(editorSelector).click({ position: { x: 24, y: 12 } });
  await page.getByRole('button', { name: 'H2' }).click();
  await expect(page.locator('#html-content')).toContainText('<h2');
  await expect(page.locator('#json-content')).toContainText('"level":2');

  await page.locator(editorSelector).click({ position: { x: 220, y: 12 } });
  await page.keyboard.type(' updated');
  await expect(page.locator('#html-content')).toContainText('updated');
  await expect(page.locator('#json-content')).toContainText('updated');

  await page.getByRole('button', { name: 'Replace content' }).click();
  await expect(page.locator('#html-content')).toContainText('Programmatic replacement');
  await expect(page.locator('#json-content')).toContainText('Programmatic replacement');
});
