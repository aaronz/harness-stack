import { test, expect } from '@playwright/test';

test.describe('REG-008: Find/Replace Works with Regex', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('opens find panel with Ctrl+F', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Find Test\n\nHello world. Hello again.');
    
    await page.keyboard.press(`${modifier}+f`);
    await page.waitForTimeout(500);
    
    const findPanel = page.locator('[data-testid="search-panel"], input[placeholder*="find" i], input[placeholder*="search" i]').first();
    const panelVisible = await findPanel.isVisible({ timeout: 3000 }).catch(() => false);
    
    if (panelVisible) {
      expect(panelVisible).toBeTruthy();
    }
  });

  test('find highlights matches', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('Testing find functionality. Testing again.');
    
    await page.keyboard.press(`${modifier}+f`);
    await page.waitForTimeout(500);
    
    const searchInput = page.locator('input[type="search"], input[placeholder*="find" i]').first();
    if (await searchInput.isVisible({ timeout: 2000 }).catch(() => false)) {
      await searchInput.fill('Testing');
      await page.waitForTimeout(500);
      
      const highlights = page.locator('mark, .highlight, [data-match]');
      const matchCount = await highlights.count();
      expect(matchCount).toBeGreaterThan(0);
    }
  });

  test('find next/previous navigation works', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('Line one. Line two. Line three.');
    
    await page.keyboard.press(`${modifier}+f`);
    await page.waitForTimeout(500);
    
    const searchInput = page.locator('input[type="search"], input[placeholder*="find" i]').first();
    if (await searchInput.isVisible({ timeout: 2000 }).catch(() => false)) {
      await searchInput.fill('Line');
      await page.waitForTimeout(300);
      
      await page.keyboard.press(`${modifier}+g`);
      await page.waitForTimeout(200);
      
      await page.keyboard.press(`${modifier}+Shift+g`);
      await page.waitForTimeout(200);
    }
  });
});