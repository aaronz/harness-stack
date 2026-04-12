import { test, expect } from '@playwright/test';

test.describe('E2E-C07: Focus Mode Writing', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('can type multiple paragraphs', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    
    for (let i = 1; i <= 5; i++) {
      await page.keyboard.type(`Paragraph ${i} with some content.`);
      await page.keyboard.press('Enter');
    }
    
    await page.waitForTimeout(500);
    
    const content = await editor.textContent();
    expect(content).toContain('Paragraph 1');
    expect(content).toContain('Paragraph 5');
  });

  test('focus mode button is available', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Test content');
    
    const focusModeButton = page.locator('button[aria-label*="focus" i], button:has-text("Focus")').first();
    const hasFocusButton = await focusModeButton.isVisible({ timeout: 2000 }).catch(() => false);
    expect(hasFocusButton).toBeTruthy();
  });

  test('typewriter mode button is available', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Test content');
    
    const typewriterButton = page.locator('button[aria-label*="typewriter" i], button:has-text("Typewriter")').first();
    const hasTypewriterButton = await typewriterButton.isVisible({ timeout: 2000 }).catch(() => false);
    expect(hasTypewriterButton).toBeTruthy();
  });
});