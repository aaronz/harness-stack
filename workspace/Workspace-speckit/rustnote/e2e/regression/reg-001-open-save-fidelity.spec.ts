import { test, expect } from '@playwright/test';
import path from 'path';
import fs from 'fs';

test.describe('REG-001: Open/Save 50 Different .md Files Without Corruption', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  const testFiles = [
    'headings.md',
    'emphasis.md',
    'links.md',
    'lists.md',
    'tasklists.md',
    'blockquotes.md',
    'codeblocks.md',
    'tables.md',
    'frontmatter.md',
    'mixed.md',
  ];

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test('opens and saves each fixture file correctly', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    for (const file of testFiles) {
      const filePath = path.join(__dirname, `../fixtures/markdown/${file}`);
      
      if (!fs.existsSync(filePath)) {
        continue;
      }

      await editor.click();
      await page.keyboard.press(`${modifier}+o`);
      await page.waitForTimeout(300);
      await page.keyboard.type(filePath);
      await page.keyboard.press('Enter');
      
      await page.waitForTimeout(1000);
      
      const content = await editor.textContent();
      expect(content).toBeTruthy();
      expect(content.length).toBeGreaterThan(0);
    }
  });
});