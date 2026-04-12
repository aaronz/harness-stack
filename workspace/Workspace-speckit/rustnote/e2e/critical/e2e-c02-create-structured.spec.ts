import { test, expect } from '@playwright/test';

test.describe('E2E-C02: Create Structured Document', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('creates document with headings and lists', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    
    await page.keyboard.type('# My Document');
    
    const h1Rendered = page.locator('h1:has-text("My Document")');
    await expect(h1Rendered).toBeVisible({ timeout: 3000 });
    
    await page.keyboard.press('Enter');
    
    await page.keyboard.type('## Introduction');
    const h2Rendered = page.locator('h2:has-text("Introduction")');
    await expect(h2Rendered).toBeVisible({ timeout: 3000 });
    
    await page.keyboard.press('Enter');
    await page.keyboard.type('- First item');
    
    const listItem = page.locator('li:has-text("First item")');
    await expect(listItem).toBeVisible({ timeout: 3000 });
    
    await page.keyboard.press('Enter');
    await page.keyboard.type('- Second item');
    
    await page.keyboard.press('Enter');
    await page.keyboard.press('Backspace');
    
    const secondListItem = page.locator('li:has-text("Second item")');
    await expect(secondListItem).toBeVisible({ timeout: 3000 });
    
    await page.keyboard.press('Enter');
    await page.keyboard.type("Here's a [link](https://example.com)");
    
    const link = page.locator('a[href="https://example.com"]:has-text("link")');
    await expect(link).toBeVisible({ timeout: 3000 });
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(500);
    
    const content = await editor.textContent();
    expect(content).toContain('# My Document');
    expect(content).toContain('- First item');
    expect(content).toContain('[link](https://example.com)');
  });

  test('Enter key auto-continues lists', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    await page.keyboard.type('- Item 1');
    await page.keyboard.press('Enter');
    
    const newListItem = page.locator('li:has-text("Item 1")');
    await expect(newListItem).toBeVisible({ timeout: 3000 });
    
    await page.keyboard.type('- Item 2');
    
    const item2 = page.locator('li:has-text("Item 2")');
    await expect(item2).toBeVisible({ timeout: 3000 });
  });

  test('Enter on empty list item exits list', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    await page.keyboard.type('- Item 1');
    await page.keyboard.press('Enter');
    await page.keyboard.press('Enter');
    
    const listCount = await page.locator('ul li').count();
    expect(listCount).toBe(1);
  });
});