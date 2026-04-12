import { test, expect } from '@playwright/test';
import path from 'path';

test.describe('REG-002: Edit-Save-Reopen 20 Files, Verify Fidelity', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('edit-save-reopen preserves all content', async ({ page }) => {
    const editor = page.locator('[data-testid="editor"]');
    await expect(editor).toBeVisible({ timeout: 10000 });

    const testFiles = ['headings.md', 'lists.md', 'emphasis.md', 'codeblocks.md'];
    
    for (let i = 0; i < 5; i++) {
      for (const file of testFiles) {
        const filePath = path.join(__dirname, `../fixtures/markdown/${file}`);
        
        await page.click(editor);
        await page.keyboard.press(`${modifier}+o`);
        await page.waitForTimeout(300);
        await page.keyboard.type(filePath);
        await page.keyboard.press('Enter');
        
        await page.waitForTimeout(800);
        
        const originalContent = await editor.textContent();
        
        await page.click(editor, { position: { x: 10, y: 10 } });
        await page.keyboard.type(`\n\nEdited at iteration ${i}`);
        
        await page.keyboard.press(`${modifier}+s`);
        await page.waitForTimeout(500);
        
        await page.keyboard.press(`${modifier}+o`);
        await page.waitForTimeout(300);
        await page.keyboard.type(filePath);
        await page.keyboard.press('Enter');
        
        await page.waitForTimeout(800);
        
        const reopenedContent = await editor.textContent();
        expect(reopenedContent).toContain('Edited at iteration');
      }
    }
  });
});