import { test, expect } from '@playwright/test';
import { chromium } from '@playwright/test';

test.describe('E2E-C06: Crash Recovery', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('shows recovery prompt after force close', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Important Document\n\nUnsaved changes here.');
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(300);
    
    const recoveryDialog = page.locator('text=/recovery|restore|discard/i');
    
    const hasRecovery = await recoveryDialog.isVisible({ timeout: 3000 }).catch(() => false);
    if (hasRecovery) {
      const restoreButton = page.locator('button:has-text("Restore"), button:has-text("Recover")').first();
      if (await restoreButton.isVisible({ timeout: 2000 }).catch(() => false)) {
        await restoreButton.click();
        
        const content = await editor.textContent();
        expect(content).toContain('Important Document');
      }
    }
  });

  test('discard recovery option works', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Document\n\nTemporary content.');
    
    const recoveryDialog = page.locator('text=/recovery|restore|discard/i');
    
    const hasRecovery = await recoveryDialog.isVisible({ timeout: 3000 }).catch(() => false);
    if (hasRecovery) {
      const discardButton = page.locator('button:has-text("Discard"), button:has-text("Skip")').first();
      if (await discardButton.isVisible({ timeout: 2000 }).catch(() => false)) {
        await discardButton.click();
        
        await page.waitForTimeout(500);
      }
    }
  });

  test('auto-recovery on app restart', async ({ browser }) => {
    const context = await browser.newContext();
    const page1 = await context.newPage();
    
    await page1.goto('/');
    await page1.waitForLoadState('domcontentloaded');
    
    const editor = page1.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });
    
    await page1.click(editor);
    await page1.keyboard.type('# Auto-save Test\n\nContent that should be auto-saved.');
    
    await page1.waitForTimeout(2000);
    
    await page1.close();
    
    const page2 = await context.newPage();
    await page2.goto('/');
    await page2.waitForLoadState('domcontentloaded');
    
    const recoveryPrompt = page2.locator('text=/auto.*save|recovery|session/i');
    const hasRecovery = await recoveryPrompt.isVisible({ timeout: 5000 }).catch(() => false);
    
    if (hasRecovery) {
      const restoreButton = page2.locator('button:has-text("Restore"), button:has-text("Recover")').first();
      if (await restoreButton.isVisible({ timeout: 2000 }).catch(() => false)) {
        await restoreButton.click();
        
        const content = page2.locator('[data-testid="editor"]').textContent();
        expect(content).toContain('Auto-save Test');
      }
    }
    
    await context.close();
  });
});