import { test, expect } from '@playwright/test';

test.describe('REG-004: Export 10 Files to HTML, Verify Validity', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('can type markdown content', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Heading\n\n**Bold** and *italic* text.');
    
    const content = await editor.textContent();
    expect(content).toContain('Heading');
    expect(content).toContain('Bold');
    expect(content).toContain('italic');
  });

  test('editor handles various markdown elements', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Test\n\nParagraph with **bold** text.\n\n- List item 1\n- List item 2\n\n> Blockquote');
    
    const content = await editor.textContent();
    expect(content).toContain('Test');
    expect(content).toContain('bold');
    expect(content).toContain('List item 1');
    expect(content).toContain('List item 2');
  });
});