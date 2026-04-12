import { test, expect } from '@playwright/test';

test.describe('E2E-C04: Technical Documentation with Code', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('creates code block with syntax highlighting', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await page.click(editor);
    
    await page.keyboard.type('```rust');
    await page.keyboard.press('Enter');
    
    await page.keyboard.type('fn main() {');
    await page.keyboard.press('Enter');
    await page.keyboard.type('    println!("Hello, world!");');
    await page.keyboard.press('Enter');
    await page.keyboard.type('}');
    await page.keyboard.press('Enter');
    await page.keyboard.type('```');
    
    const codeBlock = page.locator('pre code.language-rust, code[class*="rust"]').first();
    await expect(codeBlock).toBeVisible({ timeout: 3000 });
    
    const editorContent = await editor.textContent();
    expect(editorContent).toContain('```rust');
    expect(editorContent).toContain('fn main()');
    expect(editorContent).toContain('```');
  });

  test('language tag is preserved', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    await page.keyboard.type('```python');
    await page.keyboard.press('Enter');
    await page.keyboard.type('print("Hello")');
    await page.keyboard.press('Enter');
    await page.keyboard.type('```');
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(500);
    
    const editorContent = await editor.textContent();
    expect(editorContent).toContain('```python');
    expect(editorContent).toContain('```');
  });

  test('code block without language', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await page.click(editor);
    
    await page.keyboard.type('```');
    await page.keyboard.press('Enter');
    await page.keyboard.type('Plain code block');
    await page.keyboard.press('Enter');
    await page.keyboard.type('```');
    
    const codeBlock = page.locator('pre code').first();
    await expect(codeBlock).toBeVisible({ timeout: 3000 });
  });
});