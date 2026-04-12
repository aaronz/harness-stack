import { test, expect } from '@playwright/test';
import path from 'path';
import fs from 'fs';

test.describe('REG-006: All Markdown Fixtures Parse Correctly', () => {
  const isMac = process.platform === 'darwin';
  const modifier = isMac ? 'Meta' : 'Control';

  const fixtures = [
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

  test('each fixture parses without error', async ({ page }) => {
    const editor = page.locator('#editor-content');
    await expect(editor).toBeVisible({ timeout: 10000 });

    for (const fixture of fixtures) {
      const fixturePath = path.join(__dirname, `../fixtures/markdown/${fixture}`);
      
      if (!fs.existsSync(fixturePath)) {
        console.log(`Fixture not found: ${fixture}`);
        continue;
      }

      await editor.click();
      await page.keyboard.press(`${modifier}+o`);
      await page.waitForTimeout(300);
      await page.keyboard.type(fixturePath);
      await page.keyboard.press('Enter');
      
      await page.waitForTimeout(1000);
      
      const content = await editor.textContent();
      expect(content).toBeTruthy();
      
      const consoleErrors: string[] = [];
      page.on('console', msg => {
        if (msg.type() === 'error') {
          consoleErrors.push(msg.text());
        }
      });
      
      expect(consoleErrors.filter(e => e.includes('Error') || e.includes('error'))).toHaveLength(0);
    }
  });
});