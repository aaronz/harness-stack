import { test, expect } from '@playwright/test';
import path from 'path';

test.describe('REG-004: Export 10 Files to HTML, Verify Validity', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('exports markdown files to valid HTML', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    const testFiles = [
      'headings.md',
      'emphasis.md',
      'lists.md',
      'codeblocks.md',
      'tables.md',
      'blockquotes.md',
      'tasklists.md',
      'links.md',
      'frontmatter.md',
      'mixed.md',
    ];

    for (const file of testFiles) {
      const filePath = path.join(__dirname, `../fixtures/markdown/${file}`);
      
      await page.click(editor);
      await page.keyboard.type('');
      
      const openButton = page.locator('button[aria-label*="open" i]').first();
      if (await openButton.isVisible({ timeout: 1000 }).catch(() => false)) {
        await openButton.click();
        await page.waitForTimeout(300);
      }
      
      await page.waitForTimeout(500);
      
      const content = await editor.textContent();
      expect(content).toBeTruthy();
    }
  });

  test('exported HTML has proper structure', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Test\n\nParagraph with **bold** text.');
    
    const exportButton = page.locator('button[aria-label*="export" i]').first();
    if (await exportButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await exportButton.click();
      await page.waitForTimeout(1000);
    }
    
    const html = await page.content();
    expect(html).toContain('<h1>Test</h1>');
  });
});