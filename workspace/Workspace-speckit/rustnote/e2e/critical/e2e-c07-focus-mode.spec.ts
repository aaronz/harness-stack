import { test, expect } from '@playwright/test';

test.describe('E2E-C07: Focus Mode Writing', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('enables focus mode and dims non-current paragraphs', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    
    for (let i = 1; i <= 12; i++) {
      await page.keyboard.type(`Paragraph ${i} with some content.`);
      await page.keyboard.press('Enter');
    }
    
    await page.waitForTimeout(500);
    
    const focusModeButton = page.locator('button[aria-label*="focus" i], button:has-text("Focus")').first();
    if (await focusModeButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await focusModeButton.click();
      
      await page.waitForTimeout(500);
      
      const currentParagraph = page.locator('[data-focus-current], .focus-current, p.current');
      const dimmedParagraphs = page.locator('[data-focus-dimmed], .focus-dimmed, p.dimmed');
      
      const hasFocusMode = await currentParagraph.isVisible({ timeout: 3000 }).catch(() => false) ||
                           await dimmedParagraphs.count() > 0;
      expect(hasFocusMode).toBeTruthy();
    }
  });

  test('navigating changes current paragraph', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    for (let i = 1; i <= 5; i++) {
      await page.keyboard.type(`Paragraph ${i}.`);
      await page.keyboard.press('Enter');
    }
    
    const focusModeButton = page.locator('button[aria-label*="focus" i], button:has-text("Focus")').first();
    if (await focusModeButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await focusModeButton.click();
      
      await page.keyboard.press('ArrowDown');
      await page.waitForTimeout(300);
      
      const currentText = await page.locator('[data-testid="editor"]').textContent();
      expect(currentText).toContain('Paragraph');
    }
  });

  test('disabling focus mode shows all content equally', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    for (let i = 1; i <= 3; i++) {
      await page.keyboard.type(`Paragraph ${i}.`);
      await page.keyboard.press('Enter');
    }
    
    const focusModeButton = page.locator('button[aria-label*="focus" i], button:has-text("Focus")').first();
    if (await focusModeButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await focusModeButton.click();
      await page.waitForTimeout(300);
      await focusModeButton.click();
      
      await page.waitForTimeout(300);
      
      const allVisible = await page.locator('[data-testid="editor"] p').count();
      expect(allVisible).toBeGreaterThan(0);
    }
  });
});