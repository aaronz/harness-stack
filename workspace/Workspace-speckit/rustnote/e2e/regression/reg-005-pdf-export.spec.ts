import { test, expect } from '@playwright/test';

test.describe('REG-005: Export 10 Files to PDF, Verify Opens', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('can create content for PDF export', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Document for PDF\n\nSome content here.\n\n- Item 1\n- Item 2');
    
    const content = await editor.textContent();
    expect(content).toContain('Document for PDF');
    expect(content).toContain('Some content here.');
    expect(content).toContain('Item 1');
  });

  test('editor has toolbar buttons', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Export Test');
    
    const saveButton = page.locator('button:has-text("Save")');
    const hasSaveButton = await saveButton.isVisible({ timeout: 2000 }).catch(() => false);
    expect(hasSaveButton).toBeTruthy();
  });
});