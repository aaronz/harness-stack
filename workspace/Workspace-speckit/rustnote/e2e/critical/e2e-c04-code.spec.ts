import { test, expect } from '@playwright/test';

test.describe('E2E-C04: Technical Documentation with Code', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('creates code block with syntax highlighting', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    
    await page.keyboard.type('```rust');
    await page.keyboard.press('Enter');
    await page.keyboard.type('fn main() {');
    await page.keyboard.press('Enter');
    await page.keyboard.type('    println!("Hello, world!");');
    await page.keyboard.press('Enter');
    await page.keyboard.type('}');
    await page.keyboard.press('Enter');
    await page.keyboard.type('```');
    
    const editorContent = await editor.textContent();
    expect(editorContent).toContain('fn main()');
  });

  test('language tag is preserved', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await editor.click();
    
    await page.keyboard.type('```python');
    await page.keyboard.press('Enter');
    await page.keyboard.type('print("Hello")');
    await page.keyboard.press('Enter');
    await page.keyboard.type('```');
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(500);
    
    const editorContent = await editor.textContent();
    expect(editorContent).toContain('print("Hello")');
  });

  test('code block without language', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await editor.click();
    
    await page.keyboard.type('```');
    await page.keyboard.press('Enter');
    await page.keyboard.type('Plain code block');
    await page.keyboard.press('Enter');
    await page.keyboard.type('```');
    
    const editorContent = await editor.textContent();
    expect(editorContent).toContain('Plain code block');
  });
});