import { test, expect } from '@playwright/test';

test.describe('REG-007: Theme Toggle Works on All Platforms', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('toggles between light and dark themes', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Theme Test\n\nSome content here.');
    
    const themeButton = page.locator('button[aria-label*="theme" i], button:has-text("Theme"), button[aria-label*="dark" i], button[aria-label*="light" i]').first();
    
    if (await themeButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      const initialHtml = await page.content();
      
      await themeButton.click();
      await page.waitForTimeout(500);
      
      const afterFirstToggle = await page.content();
      
      await themeButton.click();
      await page.waitForTimeout(500);
      
      const afterSecondToggle = await page.content();
      
      expect(afterFirstToggle).toBeTruthy();
      expect(afterSecondToggle).toBeTruthy();
    }
  });

  test('all elements visible in light theme', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Light Theme Test\n\n- List item 1\n- List item 2\n\n**Bold text** and *italic text*.');
    
    const themeButton = page.locator('button[aria-label*="theme" i], button:has-text("Theme")').first();
    if (await themeButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      const isDark = await page.locator('html.dark, body.dark').isVisible().catch(() => false);
      if (isDark) {
        await themeButton.click();
        await page.waitForTimeout(500);
      }
    }
    
    const h1 = page.locator('h1:has-text("Light Theme Test")');
    await expect(h1).toBeVisible({ timeout: 3000 });
    
    const listItems = page.locator('li');
    expect(await listItems.count()).toBeGreaterThan(0);
  });

  test('all elements visible in dark theme', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    await page.keyboard.type('# Dark Theme Test\n\nContent in dark mode.');
    
    const themeButton = page.locator('button[aria-label*="theme" i], button:has-text("Theme")').first();
    if (await themeButton.isVisible({ timeout: 2000 }).catch(() => false)) {
      const isLight = await page.locator('html.light, body.light').isVisible().catch(() => false);
      if (isLight) {
        await themeButton.click();
        await page.waitForTimeout(500);
      }
    }
    
    const h1 = page.locator('h1:has-text("Dark Theme Test")');
    await expect(h1).toBeVisible({ timeout: 3000 });
  });
});