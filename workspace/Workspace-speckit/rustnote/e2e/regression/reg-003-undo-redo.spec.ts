import { test, expect } from '@playwright/test';

test.describe('REG-003: Editor History', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('editor handles text input without crash', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Some text typed into the editor.');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('Some text typed into the editor.');
  });

  test('multiple text operations do not crash', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    for (let i = 0; i < 10; i++) {
      await page.keyboard.type(`Text ${i} `);
      await page.waitForTimeout(20);
    }
    
    const content = await editor.textContent();
    expect(content).toContain('Text 0');
    expect(content).toContain('Text 9');
  });
});
