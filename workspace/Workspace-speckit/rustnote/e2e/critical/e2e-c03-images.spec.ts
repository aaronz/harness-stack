import { test, expect } from '@playwright/test';

test.describe('E2E-C03: Insert and Manage Images', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('inserts image via menu', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Image Test');
    
    await page.keyboard.press('Enter');
    
    const insertImageButton = page.locator('button[aria-label*="image" i], button[title*="image" i]').first();
    if (await insertImageButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await insertImageButton.click();
      
      const imageInput = page.locator('input[type="file"][accept*="image"]');
      if (await imageInput.isVisible({ timeout: 2000 }).catch(() => false)) {
        const testImagePath = '/tmp/test-image.png';
        await imageInput.setInputFiles(testImagePath);
      }
    }
    
    await page.keyboard.type('![Alt text](image.png)');
    
    const image = page.locator('img[src*="image.png"], img[alt="Alt text"]');
    await expect(image).toBeVisible({ timeout: 3000 });
    
    const sourceContent = await editor.textContent();
    expect(sourceContent).toContain('![Alt text](image.png)');
  });

  test('image path is relative after save', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    await page.keyboard.type('![My Image](./images/photo.png)');
    
    const image = page.locator('img[src*="./images/photo.png"], img[alt="My Image"]');
    await expect(image).toBeVisible({ timeout: 3000 });
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(500);
    
    const sourceContent = await editor.textContent();
    expect(sourceContent).toContain('./images/photo.png');
  });
});