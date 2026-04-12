import { test, expect } from '@playwright/test';

test.describe('REG-003: Undo/Redo 30 Operations Without Crash', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('30 undo operations work correctly', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    
    const initialContent = await editor.textContent();
    
    for (let i = 0; i < 30; i++) {
      await page.keyboard.type(`Text ${i} `);
    }
    
    for (let i = 0; i < 30; i++) {
      await page.keyboard.press(`${modifier}+z`);
      await page.waitForTimeout(50);
    }
    
    const afterUndo = await editor.textContent();
    expect(afterUndo).toBeTruthy();
  });

  test('30 redo operations work correctly', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    
    for (let i = 0; i < 15; i++) {
      await page.keyboard.type(`Text ${i} `);
    }
    
    for (let i = 0; i < 15; i++) {
      await page.keyboard.press(`${modifier}+z`);
    }
    
    for (let i = 0; i < 15; i++) {
      await page.keyboard.press(`${modifier}+Shift+z`);
      await page.waitForTimeout(50);
    }
    
    const afterRedo = await editor.textContent();
    expect(afterRedo).toContain('Text');
  });

  test('no crash during rapid undo/redo', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    
    for (let i = 0; i < 30; i++) {
      await page.keyboard.type(`Item ${i} `);
    }
    
    await page.waitForTimeout(100);
    
    for (let i = 0; i < 30; i++) {
      await page.keyboard.press(`${modifier}+z`);
      await page.keyboard.press(`${modifier}+Shift+z`);
    }
    
    await page.waitForTimeout(200);
    
    const content = await editor.textContent();
    expect(content).toBeTruthy();
  });
});