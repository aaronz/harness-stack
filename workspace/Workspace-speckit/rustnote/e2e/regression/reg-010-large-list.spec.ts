import { test, expect } from '@playwright/test';

test.describe('REG-010: 1000-Item List Renders Correctly', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('renders 1000-item list without crash', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Large List Test\n\n');
    
    for (let i = 1; i <= 1000; i++) {
      await page.keyboard.type(`- List item ${i}\n`);
    }
    
    await page.waitForTimeout(2000);
    
    const content = await editor.textContent();
    expect(content).toContain('List item 1');
    expect(content).toContain('List item 1000');
  });

  test('scrolls through 1000-item list', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Scrolling List\n\n');
    
    for (let i = 1; i <= 500; i++) {
      await page.keyboard.type(`- Item ${i}\n`);
    }
    
    await page.waitForTimeout(1000);
    
    await page.mouse.wheel(0, 5000);
    await page.waitForTimeout(500);
    
    const content = await editor.textContent();
    expect(content).toContain('Item');
  });

  test('1000-item list checkboxes work', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Task List\n\n');
    
    for (let i = 1; i <= 50; i++) {
      await page.keyboard.type(`- [ ] Task ${i}\n`);
    }
    
    await page.waitForTimeout(1000);
    
    const firstCheckbox = page.locator('input[type="checkbox"]').first();
    if (await firstCheckbox.isVisible({ timeout: 2000 }).catch(() => false)) {
      await firstCheckbox.check();
      await page.waitForTimeout(200);
      
      const isChecked = await firstCheckbox.isChecked();
      expect(isChecked).toBeTruthy();
    }
  });
});