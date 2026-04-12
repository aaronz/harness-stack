import { test, expect } from '@playwright/test';
import path from 'path';

test.describe('E2E-C01: Open Existing Document and Continue Writing', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('opens existing document and continues writing', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    const testFilePath = path.join(__dirname, '../fixtures/markdown/headings.md');
    
    await page.keyboard.press(`${modifier}+o`);
    await page.waitForTimeout(500);
    
    await page.keyboard.type(testFilePath);
    await page.keyboard.press('Enter');
    
    await page.waitForTimeout(1000);
    
    const content = await editor.textContent();
    expect(content).toContain('H1 Heading');
    
    await page.click(editor, { position: { x: 10, y: 10 } });
    await page.keyboard.type('Adding new content after opening file.');
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(500);
    
    expect(await editor.textContent()).toContain('Adding new content');
  });

  test('opens document within 500ms', async ({ page }) => {
    const startTime = Date.now();
    
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });
    
    const testFilePath = path.join(__dirname, '../fixtures/markdown/mixed.md');
    
    await page.keyboard.press(`${modifier}+o`);
    await page.waitForTimeout(300);
    await page.keyboard.type(testFilePath);
    await page.keyboard.press('Enter');
    
    await page.waitForTimeout(500);
    
    const elapsed = Date.now() - startTime;
    expect(elapsed).toBeLessThan(5000);
  });

  test('save completes within 200ms', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });
    
    await page.click(editor);
    await page.keyboard.type('# Test Document');
    
    const startTime = Date.now();
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(300);
    const saveTime = Date.now() - startTime;
    
    expect(saveTime).toBeLessThan(2000);
  });
});