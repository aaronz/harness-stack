import { test, expect } from '@playwright/test';

test.describe('Editor CSS Style Verification', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
  });

  test.describe('Headings', () => {
    test('H1 has correct font-size and font-weight', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('# Test Heading');
      await page.waitForTimeout(300);

      const h1 = page.locator('#editor-content h1');
      await expect(h1).toBeVisible();
      await expect(h1).toHaveCSS('font-size', '32px');
      await expect(h1).toHaveCSS('font-weight', '600');
      await expect(h1).toHaveCSS('line-height', '41.6px');
    });

    test('H2 has correct font-size and font-weight', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('## Test Heading 2');
      await page.waitForTimeout(300);

      const h2 = page.locator('#editor-content h2');
      await expect(h2).toBeVisible();
      await expect(h2).toHaveCSS('font-size', '24px');
      await expect(h2).toHaveCSS('font-weight', '600');
    });

    test('H3 has correct font-size and font-weight', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('### Test Heading 3');
      await page.waitForTimeout(300);

      const h3 = page.locator('#editor-content h3');
      await expect(h3).toBeVisible();
      await expect(h3).toHaveCSS('font-size', '20px');
      await expect(h3).toHaveCSS('font-weight', '600');
    });
  });

  test.describe('Text Formatting', () => {
    test('bold text has correct font-weight', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('**bold text**');
      await page.waitForTimeout(300);

      const bold = page.locator('#editor-content strong, #editor-content b').first();
      await expect(bold).toBeVisible();
      await expect(bold).toHaveCSS('font-weight', '700');
    });

    test('italic text has correct font-style', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('*italic text*');
      await page.waitForTimeout(300);

      const italic = page.locator('#editor-content em, #editor-content i').first();
      await expect(italic).toBeVisible();
      await expect(italic).toHaveCSS('font-style', 'italic');
    });
  });

  test.describe('Lists', () => {
    test('unordered list has disc markers', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('- Item 1');
      await page.waitForTimeout(300);

      const ul = page.locator('#editor-content ul');
      await expect(ul).toBeVisible();
      await expect(ul).toHaveCSS('list-style-type', 'disc');
    });

    test('ordered list has decimal markers', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('1. Item 1');
      await page.waitForTimeout(300);

      const ol = page.locator('#editor-content ol');
      await expect(ol).toBeVisible();
      await expect(ol).toHaveCSS('list-style-type', 'decimal');
    });

    test('list item has correct margins', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('- Item 1');
      await page.waitForTimeout(300);

      const li = page.locator('#editor-content li').first();
      await expect(li).toBeVisible();
      await expect(li).toHaveCSS('margin-top', '8px');
      await expect(li).toHaveCSS('margin-bottom', '8px');
    });
  });

  test.describe('Blockquote', () => {
    test('blockquote has correct border and padding', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('> quoted text');
      await page.waitForTimeout(300);

      const blockquote = page.locator('#editor-content blockquote');
      await expect(blockquote).toBeVisible();
      await expect(blockquote).toHaveCSS('border-left-width', '4px');
      await expect(blockquote).toHaveCSS('padding-left', '16px');
    });
  });

  test.describe('Code', () => {
    test('inline code has correct font and background', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('`inline code`');
      await page.waitForTimeout(300);

      const code = page.locator('#editor-content code').first();
      await expect(code).toBeVisible();
      await expect(code).toHaveCSS('font-family', /mono/);
      await expect(code).toHaveCSS('background-color', 'rgb(245, 245, 245)');
    });
  });

  });
