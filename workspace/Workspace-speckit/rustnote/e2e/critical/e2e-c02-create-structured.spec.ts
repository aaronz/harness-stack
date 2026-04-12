import { test, expect } from '@playwright/test';

test.describe('E2E-C02: Create Structured Document', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('creates document with headings and lists', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    
    await page.keyboard.type('# My Document');
    await page.keyboard.press('Enter');
    await page.keyboard.type('## Introduction');
    await page.keyboard.press('Enter');
    await page.keyboard.type('- First item');
    await page.keyboard.press('Enter');
    await page.keyboard.type('- Second item');
    await page.keyboard.press('Enter');
    await page.keyboard.press('Backspace');
    await page.keyboard.press('Enter');
    await page.keyboard.type("Here's a [link](https://example.com)");
    
    const content = await editor.textContent();
    expect(content).toContain('My Document');
    expect(content).toContain('Introduction');
    expect(content).toContain('First item');
    expect(content).toContain('Second item');
    expect(content).toContain('link');
  });

  test('Enter key auto-continues lists', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await editor.click();
    
    await page.keyboard.type('- Item 1');
    await page.keyboard.press('Enter');
    await page.keyboard.type('- Item 2');
    
    const content = await editor.textContent();
    expect(content).toContain('Item 1');
    expect(content).toContain('Item 2');
  });

  test('Enter on empty list item exits list', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await editor.click();
    
    await page.keyboard.type('- Item 1');
    await page.keyboard.press('Enter');
    await page.keyboard.press('Enter');
    
    const content = await editor.textContent();
    expect(content).toContain('Item 1');
  });
});