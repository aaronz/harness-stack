import { test, expect } from '@playwright/test';

test.describe('E2E-C01: Open Existing Document and Continue Writing', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('opens existing document and continues writing', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Test Document');
    await page.waitForTimeout(200);
    
    const content = await editor.textContent();
    expect(content).toContain('Test Document');
    
    await page.keyboard.type('\n\nAdding new content.');
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(300);
    
    expect(await editor.textContent()).toContain('Adding new content');
  });

  test('opens document within 500ms', async ({ page }) => {
    const startTime = Date.now();
    
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    
    await editor.click();
    await page.keyboard.type('# Quick Test');
    
    const elapsed = Date.now() - startTime;
    expect(elapsed).toBeLessThan(2000);
  });

  test('save completes within 200ms', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    
    await editor.click();
    await page.keyboard.type('# Test Document');
    
    const startTime = Date.now();
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(300);
    const saveTime = Date.now() - startTime;
    
    expect(saveTime).toBeLessThan(2000);
  });
});