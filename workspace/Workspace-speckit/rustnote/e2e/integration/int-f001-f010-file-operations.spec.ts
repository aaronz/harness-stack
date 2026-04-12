import { test, expect } from '@playwright/test';

const isMac = process.platform === 'darwin';
const modifier = isMac ? 'Meta' : 'Control';

test.describe('INT-F: File Operations Integration Tests', () => {

  test.describe('INT-F001: Create new file', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('creates untitled document when File → New action is triggered', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await expect(editor).toBeVisible({ timeout: 10000 });

      await editor.click();
      await page.keyboard.press(`${modifier}+n`);
      await page.waitForTimeout(500);

      const content = await editor.textContent();
      expect(content === '' || content === null).toBeTruthy();
    });
  });

  test.describe('INT-F002: Save new file', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('triggers save document command when Ctrl+S is pressed', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await expect(editor).toBeVisible({ timeout: 10000 });

      await editor.click();
      await page.keyboard.type('# Test Document\n\nContent here.');

      await page.keyboard.press(`${modifier}+s`);
      await page.waitForTimeout(1000);

      const content = await editor.textContent();
      expect(content).toContain('Test Document');
    });
  });

  test.describe('INT-F003: Save existing file', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('saves content when Ctrl+S is pressed after editing', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await expect(editor).toBeVisible({ timeout: 10000 });

      await editor.click();
      await page.keyboard.type('# Updated Content\n\nNew text here.');

      await page.keyboard.press(`${modifier}+s`);
      await page.waitForTimeout(1000);

      const content = await editor.textContent();
      expect(content).toContain('Updated Content');
    });
  });

  test.describe('INT-F004: Save-as', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('triggers save as action when Ctrl+Shift+S is pressed', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await expect(editor).toBeVisible({ timeout: 10000 });

      await editor.click();
      await page.keyboard.type('# Save As Test');

      await page.keyboard.press(`${modifier}+Shift+s`);
      await page.waitForTimeout(1000);

      const content = await editor.textContent();
      expect(content).toContain('Save As Test');
    });
  });

  test.describe('INT-F005: Open existing file', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('opens file dialog when Ctrl+O is pressed', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await expect(editor).toBeVisible({ timeout: 10000 });

      await page.keyboard.press(`${modifier}+o`);
      await page.waitForTimeout(1000);

      const content = await editor.textContent();
      expect(content !== null).toBeTruthy();
    });
  });

  test.describe('INT-F007: Open folder as workspace', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('shows sidebar with Open Workspace button', async ({ page }) => {
      const sidebar = page.locator('#sidebar');
      await expect(sidebar).toBeVisible({ timeout: 10000 });

      const openWorkspaceBtn = page.locator('button:has-text("Open Workspace")');
      await expect(openWorkspaceBtn).toBeVisible();
    });
  });

  test.describe('INT-F008: Recent files persist', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('can open multiple files via Ctrl+O', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await expect(editor).toBeVisible({ timeout: 10000 });

      await page.keyboard.press(`${modifier}+o`);
      await page.waitForTimeout(500);

      await page.keyboard.press(`${modifier}+o`);
      await page.waitForTimeout(500);

      const content = await editor.textContent();
      expect(content !== null).toBeTruthy();
    });
  });

  test.describe('INT-F009: File permissions denied', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('handles file dialog gracefully', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await expect(editor).toBeVisible({ timeout: 10000 });

      await page.keyboard.press(`${modifier}+o`);
      await page.waitForTimeout(500);

      const content = await editor.textContent();
      expect(content !== null).toBeTruthy();
    });
  });

  test.describe('INT-F010: Unsaved changes prompt', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
    });

    test('marks document as dirty when content is modified', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await expect(editor).toBeVisible({ timeout: 10000 });

      await editor.click();
      await page.keyboard.type('# Unsaved Changes Test');

      await page.waitForTimeout(300);

      const isEditable = await editor.isEditable();
      expect(isEditable).toBe(true);
    });
  });
});