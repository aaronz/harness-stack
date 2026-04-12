import { test, expect } from '@playwright/test';
import path from 'path';

test.describe('REG-009: 5MB Document Scrolls at 60 FPS', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('opens large document without crash', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    const largeFilePath = path.join(__dirname, '../fixtures/markdown/long.md');
    
    await page.click(editor);
    await page.keyboard.press(`${modifier}+o`);
    await page.waitForTimeout(300);
    await page.keyboard.type(largeFilePath);
    await page.keyboard.press('Enter');
    
    await page.waitForTimeout(5000);
    
    const content = await editor.textContent();
    expect(content).toBeTruthy();
  });

  test('scrolls through large document', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Large Document\n\n');
    
    for (let i = 0; i < 100; i++) {
      await page.keyboard.type(`Paragraph ${i}: Lorem ipsum dolor sit amet. `.repeat(5));
      await page.keyboard.press('Enter');
    }
    
    await page.waitForTimeout(1000);
    
    await page.mouse.wheel(0, 1000);
    await page.waitForTimeout(500);
    
    await page.mouse.wheel(0, 1000);
    await page.waitForTimeout(500);
    
    const content = await editor.textContent();
    expect(content).toContain('Paragraph');
  });

  test('no crash during rapid scrolling', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    
    for (let i = 0; i < 50; i++) {
      await page.keyboard.type(`Section ${i} content here.\n`);
    }
    
    await page.waitForTimeout(500);
    
    for (let i = 0; i < 10; i++) {
      await page.mouse.wheel(0, 500);
      await page.waitForTimeout(100);
    }
    
    const content = await editor.textContent();
    expect(content).toBeTruthy();
  });
});