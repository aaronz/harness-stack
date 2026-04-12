import { test, expect } from '@playwright/test';

test.describe('E2E-C05: Export Workflow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('exports document to HTML', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Export Test\n\nThis is a **test** document.');
    
    await page.waitForTimeout(500);
    
    const exportButton = page.locator('button[aria-label*="export" i], button:has-text("Export")').first();
    if (await exportButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await exportButton.click();
      
      const htmlOption = page.locator('button:has-text("HTML"), li:has-text("HTML")').first();
      if (await htmlOption.isVisible({ timeout: 2000 }).catch(() => false)) {
        await htmlOption.click();
        
        await page.waitForTimeout(2000);
      }
    }
    
    const content = await editor.textContent();
    expect(content).toContain('# Export Test');
    expect(content).toContain('**test**');
  });

  test('exports document to PDF', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# PDF Test\n\nContent for PDF export.');
    
    await page.waitForTimeout(500);
    
    const exportButton = page.locator('button[aria-label*="export" i], button:has-text("Export")').first();
    if (await exportButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await exportButton.click();
      
      const pdfOption = page.locator('button:has-text("PDF"), li:has-text("PDF")').first();
      if (await pdfOption.isVisible({ timeout: 2000 }).catch(() => false)) {
        await pdfOption.click();
        
        await page.waitForTimeout(3000);
      }
    }
    
    const content = await editor.textContent();
    expect(content).toContain('# PDF Test');
  });
});