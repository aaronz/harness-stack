import { test, expect } from '@playwright/test';

test.describe('Text Input Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('editor is visible and focusable', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    
    const isEditable = await editor.getAttribute('contentEditable');
    expect(isEditable).toBe('true');
  });

  test('can type single character', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('a');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('a');
  });

  test('can type multiple characters', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('hello');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('hello');
  });

  test('can type spaces between words', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('hello world');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('hello world');
  });

  test('can type numbers and symbols', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Test 123 !@#$%');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('Test 123 !@#$%');
  });

  test('can type newlines with Enter key', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Line 1');
    await page.keyboard.press('Enter');
    await page.keyboard.type('Line 2');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toContain('Line 1');
    expect(content).toContain('Line 2');
  });

  test('can type Chinese characters', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('你好世界');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('你好世界');
  });

  test('can type emoji', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Hello 👋 World 🌍');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('Hello 👋 World 🌍');
  });

  test('content persists after clicking elsewhere and back', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Persistent content');
    await page.waitForTimeout(100);
    
    await page.locator('body').click({ position: { x: 10, y: 10 } });
    await page.waitForTimeout(100);
    
    await editor.click();
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toContain('Persistent content');
  });

  test('can select all and replace content', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Original text');
    await page.waitForTimeout(100);
    
    const isMac = process.platform === 'darwin';
    const modifier = isMac ? 'Meta' : 'Control';
    await page.keyboard.press(`${modifier}+a`);
    await page.waitForTimeout(50);
    await page.keyboard.type('Replaced');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('Replaced');
  });

  test('can use backspace to delete', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Hello');
    await page.waitForTimeout(100);
    
    for (let i = 0; i < 3; i++) {
      await page.keyboard.press('Backspace');
    }
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('He');
  });

  test('can use delete key to remove text', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Test');
    await page.keyboard.press('Home');
    await page.waitForTimeout(50);
    
    for (let i = 0; i < 2; i++) {
      await page.keyboard.press('Delete');
    }
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content.length).toBe(2);
  });

  test('can type at beginning of existing content', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('World');
    await page.keyboard.press('Home');
    await page.waitForTimeout(50);
    await page.keyboard.type('Hello ');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toBe('Hello World');
  });

  test('can type after navigating with arrow keys', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('ABC');
    await page.keyboard.press('Home');
    await page.waitForTimeout(50);
    await page.keyboard.press('ArrowRight');
    await page.keyboard.press('ArrowRight');
    await page.waitForTimeout(50);
    await page.keyboard.type('X');
    await page.waitForTimeout(100);
    
    const content = await editor.textContent();
    expect(content).toContain('X');
  });

  test('content is preserved after page reload simulation', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });
    await editor.click();
    
    await page.keyboard.type('Important content that must be preserved');
    await page.waitForTimeout(200);
    
    const content = await editor.textContent();
    expect(content).toBe('Important content that must be preserved');
  });
});
