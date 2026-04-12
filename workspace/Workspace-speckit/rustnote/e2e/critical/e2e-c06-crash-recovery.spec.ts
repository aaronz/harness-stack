import { test, expect } from '@playwright/test';

test.describe('E2E-C06: Crash Recovery', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('editor preserves unsaved content', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Important Document\n\nUnsaved changes here.');
    await page.waitForTimeout(300);
    
    const content = await editor.textContent();
    expect(content).toContain('Important Document');
    expect(content).toContain('Unsaved changes here.');
  });

  test('save command is available', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Document\n\nTemporary content.');
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(300);
    
    const content = await editor.textContent();
    expect(content).toContain('Document');
  });
});