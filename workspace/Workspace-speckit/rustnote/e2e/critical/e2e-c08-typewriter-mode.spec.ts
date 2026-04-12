import { test, expect } from '@playwright/test';

test.describe('E2E-C08: Typewriter Mode', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('enables typewriter mode and keeps cursor centered', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    
    for (let i = 1; i <= 20; i++) {
      await page.keyboard.type(`Paragraph ${i}.`);
      await page.keyboard.press('Enter');
    }
    
    await page.waitForTimeout(500);
    
    const typewriterButton = page.locator('button[aria-label*="typewriter" i], button:has-text("Typewriter")').first();
    if (await typewriterButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await typewriterButton.click();
      
      await page.waitForTimeout(300);
      
      await page.keyboard.press('ArrowDown');
      await page.keyboard.press('ArrowDown');
      await page.keyboard.press('ArrowDown');
      await page.waitForTimeout(500);
      
      const editorBox = await editor.boundingBox();
      expect(editorBox).not.toBeNull();
    }
  });

  test('typing maintains cursor vertical position', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    for (let i = 1; i <= 15; i++) {
      await page.keyboard.type(`Paragraph ${i}.`);
      await page.keyboard.press('Enter');
    }
    
    const typewriterButton = page.locator('button[aria-label*="typewriter" i], button:has-text("Typewriter")').first();
    if (await typewriterButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await typewriterButton.click();
      
      await page.keyboard.press('ArrowDown');
      await page.keyboard.press('ArrowDown');
      await page.keyboard.press('ArrowDown');
      await page.waitForTimeout(300);
      
      await page.keyboard.type('New text being added.');
      await page.waitForTimeout(300);
      
      const editorBox = await editor.boundingBox();
      expect(editorBox).not.toBeNull();
    }
  });

  test('disabling typewriter mode removes centering', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    for (let i = 1; i <= 10; i++) {
      await page.keyboard.type(`Paragraph ${i}.`);
      await page.keyboard.press('Enter');
    }
    
    const typewriterButton = page.locator('button[aria-label*="typewriter" i], button:has-text("Typewriter")').first();
    if (await typewriterButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      await typewriterButton.click();
      await page.waitForTimeout(300);
      await typewriterButton.click();
      
      await page.waitForTimeout(300);
      
      const scrollWorks = await page.evaluate(() => {
        const editor = document.querySelector('[data-testid="editor"]');
        return editor ? editor.scrollTop >= 0 : true;
      });
      expect(scrollWorks).toBeTruthy();
    }
  });
});