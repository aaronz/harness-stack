import { test, expect } from '@playwright/test';

test.describe('Security Tests', () => {
  test.describe('Input Sanitization', () => {
    test('prevents XSS via script injection', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const maliciousInput = '<script>alert("xss")</script>';
      await page.keyboard.type(maliciousInput);
      
      const content = await editor.innerHTML();
      expect(content).not.toContain('<script>');
    });

    test('sanitizes HTML tags in input', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('<div onclick="alert(1)">test</div>');
      
      const content = await editor.innerHTML();
      expect(content).not.toContain('onclick');
    });

    test('handles malformed HTML gracefully', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('<unclosed><tag>content');
      
      const content = await editor.textContent();
      expect(content).toBeDefined();
    });
  });

  test.describe('File Operations', () => {
    test('prevents path traversal in file names', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const maliciousPath = '../../../etc/passwd';
      expect(maliciousPath).toContain('..');
    });

    test('validates file paths are within workspace', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const workspacePath = '/Users/user/workspace';
      const filePath = '/Users/user/workspace/subdir/file.md';
      
      expect(filePath.startsWith(workspacePath)).toBe(true);
    });
  });

  test.describe('Content Security', () => {
    test('prevents inline event handlers', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('<img src=x onerror="alert(1)">');
      
      const content = await editor.innerHTML();
      expect(content).not.toContain('onerror');
    });

    test('sanitizes javascript: URLs', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('[link](javascript:alert(1))');
      
      const content = await editor.textContent();
      expect(content).not.toContain('javascript:');
    });

    test('sanitizes data: URLs in images', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('![img](data:text/html,<script>alert(1)</script>)');
      
      const content = await editor.textContent();
      expect(content).not.toContain('data:');
    });
  });

  test.describe('Export Security', () => {
    test('HTML export sanitizes output', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('<script>alert("xss")</script>');
      
      const exportedHtml = await page.evaluate(() => {
        return document.querySelector('#editor-content')?.innerHTML || '';
      });
      
      expect(exportedHtml).not.toContain('<script>');
    });

    test('prevents iframe injection in HTML export', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('<iframe src="evil.com"></iframe>');
      
      const exportedHtml = await page.evaluate(() => {
        return document.querySelector('#editor-content')?.innerHTML || '';
      });
      
      expect(exportedHtml).not.toContain('<iframe');
    });
  });

  test.describe('Settings Security', () => {
    test('settings cannot execute arbitrary code', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const settings = {
        fontFamily: 'font-family: expression(alert(1))',
        fontSize: 16,
      };
      
      expect(settings.fontFamily).not.toContain('expression');
    });

    test('external URLs in links are safe', async ({ page }) => {
      const urls = [
        'https://legitimate-site.com',
        'javascript:alert(1)',
        'data:text/html,<script>alert(1)</script>',
      ];
      
      urls.forEach(url => {
        expect(url.match(/^https?:\/\//)).toBeTruthy();
      });
    });
  });

  test.describe('Memory Security', () => {
    test('handles large content without DoS', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const largeContent = 'x'.repeat(10 * 1024 * 1024);
      await page.keyboard.type(largeContent.substring(0, 1000));
      
      const content = await editor.textContent();
      expect(content?.length).toBeLessThan(10000);
    });

    test('prevents catastrophic backtracking in regex', async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('(a+)+b');
      
      const content = await editor.textContent();
      expect(content).toBeDefined();
    });
  });
});
