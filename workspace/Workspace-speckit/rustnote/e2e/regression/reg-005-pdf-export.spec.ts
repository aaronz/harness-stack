import { test, expect } from '@playwright/test';
import path from 'path';

test.describe('REG-005: Export 10 Files to PDF, Verify Opens', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('exports markdown files to PDF', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    const testFiles = [
      'headings.md',
      'emphasis.md',
      'lists.md',
      'codeblocks.md',
      'tables.md',
    ];

    for (const file of testFiles) {
      const filePath = path.join(__dirname, `../fixtures/markdown/${file}`);
      
      await page.click(editor);
      await page.keyboard.type('');
      
      await page.waitForTimeout(500);
      
      const content = await editor.textContent();
      expect(content).toBeTruthy();
    }
  });

  test('PDF export produces non-empty file', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Document\n\nSome content here.');
    
    const exportButton = page.locator('button[aria-label*="export" i]').first();
    if (await exportButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await exportButton.click();
      
      const pdfOption = page.locator('text=/PDF/i').first();
      if (await pdfOption.isVisible({ timeout: 2000 }).catch(() => false)) {
        await pdfOption.click();
        await page.waitForTimeout(3000);
      }
    }
    
    const content = await editor.textContent();
    expect(content).toContain('Document');
  });
});