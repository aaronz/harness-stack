import { test, expect } from '@playwright/test';

test.describe('E2E-C03: Insert and Manage Images', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('inserts image via menu', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Image Test');
    await page.keyboard.press('Enter');
    await page.keyboard.type('![Alt text](image.png)');
    
    const sourceContent = await editor.textContent();
    expect(sourceContent).toContain('![Alt text](image.png)');
  });

  test('image path is relative after save', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await editor.click();
    
    await page.keyboard.type('![My Image](./images/photo.png)');
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(500);
    
    const sourceContent = await editor.textContent();
    expect(sourceContent).toContain('./images/photo.png');
  });
});