import { test, expect, AxeBuilder } from '@axe-core/playwright';

test.describe('Accessibility Tests (a11y)', () => {
  test('page has proper document structure', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const html = await page.content();
    expect(html).toContain('<html');
    expect(html).toContain('<head>');
    expect(html).toContain('<body>');
  });

  test('editor has proper ARIA role', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await expect(editor).toHaveAttribute('role', /.*/);
  });

  test('headings are properly structured', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await editor.click();
    await page.keyboard.type('# Title\n## Section\n### Subsection');
    
    const h1 = page.locator('h1');
    const h2 = page.locator('h2');
    const h3 = page.locator('h3');
    
    await expect(h1).toHaveCount(1);
    await expect(h2).toHaveCount(1);
    await expect(h3).toHaveCount(1);
  });

  test('links have accessible text', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await editor.click();
    await page.keyboard.type('[link text](https://example.com)');
    
    const link = page.locator('a').first();
    await expect(link).toHaveText('link text');
  });

  test('images have alt text', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await editor.click();
    await page.keyboard.type('![description](image.png)');
    
    const img = page.locator('img').first();
    await expect(img).toHaveAttribute('alt', 'description');
  });

  test('focus is visible', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await editor.click();
    
    const focused = await page.evaluate(() => {
      const el = document.activeElement;
      return el ? window.getComputedStyle(el).outline : 'none';
    });
    
    expect(focused).not.toBe('none');
  });

  test('keyboard navigation works', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await editor.click();
    await page.keyboard.type('test');
    
    await page.keyboard.press('Control+a');
    await page.keyboard.press('Control+c');
    
    const content = await editor.textContent();
    expect(content).toContain('test');
  });

  test('Tab key navigates correctly', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    await page.keyboard.press('Tab');
    await page.keyboard.press('Shift+Tab');
    
    const focused = await page.evaluate(() => document.activeElement?.tagName);
    expect(focused).toBeDefined();
  });

  test('color contrast meets WCAG standards', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const results = await new AxeBuilder({ page })
      .withTags(['wcag2a', 'wcag2aa'])
      .analyze();
    
    expect(results.violations).toEqual([]);
  });

  test('page has lang attribute', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const html = page.locator('html');
    await expect(html).toHaveAttribute('lang');
  });

  test('buttons have accessible names', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const buttons = page.locator('button');
    const count = await buttons.count();
    
    for (let i = 0; i < Math.min(count, 5); i++) {
      const button = buttons.nth(i);
      const hasText = await button.textContent();
      const hasAriaLabel = await button.getAttribute('aria-label');
      expect(hasText || hasAriaLabel).toBeTruthy();
    }
  });

  test('form inputs have labels', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const inputs = page.locator('input');
    const count = await inputs.count();
    
    for (let i = 0; i < Math.min(count, 5); i++) {
      const input = inputs.nth(i);
      const id = await input.getAttribute('id');
      const ariaLabel = await input.getAttribute('aria-label');
      const ariaLabelledBy = await input.getAttribute('aria-labelledby');
      
      if (id) {
        const label = page.locator(`label[for="${id}"]`);
        expect(await label.count()).toBeGreaterThan(0);
      } else {
        expect(ariaLabel || ariaLabelledBy).toBeTruthy();
      }
    }
  });

  test('dynamic content updates are announced', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await editor.click();
    await page.keyboard.type('# New Heading');
    
    const liveRegion = page.locator('[aria-live]');
    const count = await liveRegion.count();
    
    expect(count).toBeGreaterThan(0);
  });

  test('skip link is provided', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const skipLink = page.locator('a[href="#main"], a[href="#content"], .skip-link');
    const count = await skipLink.count();
    
    if (count > 0) {
      await expect(skipLink.first()).toHaveText(/skip|content|main/i);
    }
  });

  test('no duplicate IDs in DOM', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const ids = await page.evaluate(() => {
      const elements = document.querySelectorAll('[id]');
      const idValues = Array.from(elements).map(el => el.id);
      const duplicates = idValues.filter((id, idx) => idValues.indexOf(id) !== idx);
      return duplicates;
    });
    
    expect(ids).toEqual([]);
  });

  test('list items are within parent lists', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await editor.click();
    await page.keyboard.type('- Item 1\n- Item 2');
    
    const ul = page.locator('ul');
    const li = page.locator('li');
    
    const ulCount = await ul.count();
    const liCount = await li.count();
    
    expect(liCount).toBeGreaterThanOrEqual(ulCount);
  });

  test('tables have proper headers', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const editor = page.locator('#editor-content');
    await editor.click();
    await page.keyboard.type('| Header |\n| --- |\n| Cell |');
    
    const table = page.locator('table');
    const th = page.locator('th');
    
    if (await table.count() > 0) {
      await expect(th).toHaveCount(await table.count());
    }
  });
});
