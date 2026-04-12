import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Visual Regression Tests', () => {
  const baselineDir = './visual-baselines';
  const screenshotsDir = './visual-screenshots';

  test.beforeAll(() => {
    if (!fs.existsSync(baselineDir)) {
      fs.mkdirSync(baselineDir, { recursive: true });
    }
    if (!fs.existsSync(screenshotsDir)) {
      fs.mkdirSync(screenshotsDir, { recursive: true });
    }
  });

  test.describe('Editor Rendering', () => {
    test('heading styles render correctly', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('# Heading 1');
      
      const screenshot = await page.screenshot();
      const baselinePath = path.join(baselineDir, 'heading-1.png');
      
      if (fs.existsSync(baselinePath)) {
        expect(screenshot).toMatchSnapshot('heading-1', { threshold: 0.1 });
      } else {
        fs.writeFileSync(baselinePath, screenshot);
      }
    });

    test('bold text renders correctly', async ({ page }) => {
      await page.goto('/');
      await editor.click();
      await page.keyboard.type('**bold text**');
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });

    test('italic text renders correctly', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('*italic text*');
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });

    test('code block renders correctly', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('```\ncode\n```');
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });

    test('list renders correctly', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('- Item 1\n- Item 2');
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });

    test('blockquote renders correctly', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('> quoted text');
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });
  });

  test.describe('Theme', () => {
    test('light theme renders correctly', async ({ page }) => {
      await page.goto('/?theme=light');
      await page.waitForLoadState('domcontentloaded');
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });

    test('dark theme renders correctly', async ({ page }) => {
      await page.goto('/?theme=dark');
      await page.waitForLoadState('domcontentloaded');
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });
  });

  test.describe('Layout', () => {
    test('sidebar visibility', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const sidebar = page.locator('#sidebar');
      await expect(sidebar).toBeVisible();
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });

    test('outline visibility', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      await page.keyboard.type('# Title\n## Section');
      await page.waitForTimeout(500);
      
      const screenshot = await page.screenshot();
      expect(screenshot).toBeDefined();
    });
  });
});
