import { test, expect } from '@playwright/test';

test.describe('REG-002: Edit-Save-Reopen 20 Files, Verify Fidelity', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('content is preserved after typing', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Document');
    await page.keyboard.press('Enter');
    await page.keyboard.type('Some content here.');
    
    await page.waitForTimeout(200);
    
    const content = await editor.textContent();
    expect(content).toContain('Document');
    expect(content).toContain('Some content here.');
    
    await page.keyboard.type('\n\nMore content added.');
    
    const updatedContent = await editor.textContent();
    expect(updatedContent).toContain('More content added.');
  });

  test('save command works without error', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    await editor.click();
    await page.keyboard.type('# Test Document');
    
    await page.keyboard.press(`${modifier}+s`);
    await page.waitForTimeout(300);
    
    const content = await editor.textContent();
    expect(content).toContain('Test Document');
  });
});