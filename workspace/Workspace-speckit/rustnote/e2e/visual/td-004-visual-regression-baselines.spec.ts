/**
 * Visual Regression Baselines Test Suite
 * 
 * This test suite establishes and verifies visual regression baselines for key editor states.
 * Each test case captures a screenshot and compares it against stored baselines.
 * 
 * Test Cases:
 * - TC-VR001: Empty editor baseline
 * - TC-VR002: Focused editor with content baseline
 * - TC-VR003: Dark theme baseline
 * - TC-VR004: Light theme baseline
 * - TC-VR005: Focus mode baseline
 * - TC-VR006: Typewriter mode baseline
 * - TC-VR007: Export modal baseline
 * - TC-VR008: Visual regression CI check
 * 
 * Run with: npm run test:visual
 */

import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

const BASELINE_DIR = './visual-baselines';
const SCREENSHOTS_DIR = './visual-screenshots';

// Ensure directories exist
test.beforeAll(() => {
  if (!fs.existsSync(BASELINE_DIR)) {
    fs.mkdirSync(BASELINE_DIR, { recursive: true });
  }
  if (!fs.existsSync(SCREENSHOTS_DIR)) {
    fs.mkdirSync(SCREENSHOTS_DIR, { recursive: true });
  }
});

/**
 * Helper function to capture screenshot with baseline management
 * Creates baseline if it doesn't exist, compares otherwise
 */
async function captureBaseline(
  page: any,
  testName: string,
  options: { fullPage?: boolean; threshold?: number } = {}
): Promise<{ passed: boolean; baselineExists: boolean }> {
  const { fullPage = true, threshold = 0.1 } = options;
  const baselinePath = path.join(BASELINE_DIR, `${testName}.png`);
  const screenshot = await page.screenshot({ fullPage });
  
  if (fs.existsSync(baselinePath)) {
    // Compare with baseline
    const baseline = fs.readFileSync(baselinePath);
    const isMatch = screenshot.equals(baseline) || 
      compareImages(screenshot, baseline, threshold);
    
    // Save current screenshot for comparison
    const currentPath = path.join(SCREENSHOTS_DIR, `${testName}-current.png`);
    fs.writeFileSync(currentPath, screenshot);
    
    return { passed: isMatch, baselineExists: true };
  } else {
    // Create new baseline
    fs.writeFileSync(baselinePath, screenshot);
    return { passed: true, baselineExists: false };
  }
}

/**
 * Simple image comparison using buffer comparison
 * For more sophisticated comparison, consider using pixelmatch or resemble.js
 */
function compareImages(img1: Buffer, img2: Buffer, threshold: number): boolean {
  if (img1.length !== img2.length) {
    return false;
  }
  
  // Simple byte comparison as fallback
  let differences = 0;
  for (let i = 0; i < img1.length; i++) {
    if (img1[i] !== img2[i]) {
      differences++;
    }
  }
  
  const diffRatio = differences / img1.length;
  return diffRatio <= threshold;
}

test.describe('Visual Regression Baselines - TD-004', () => {
  
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForSelector('#editor-content', { timeout: 10000 });
  });

  /**
   * TC-VR001: Empty editor baseline
   * Category: render
   * Input: Fresh editor, no content
   * Expected: Screenshot captured for baseline
   */
  test.describe('TC-VR001: Empty editor baseline', () => {
    test('should capture empty editor baseline', async ({ page }) => {
      // Ensure editor is empty
      const editor = page.locator('#editor-content');
      await editor.click();
      
      // Clear any existing content
      await page.keyboard.press('Control+a');
      await page.keyboard.press('Backspace');
      
      await page.waitForTimeout(500);
      
      const result = await captureBaseline(page, 'vr001-empty-editor', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR001: Empty editor');
      }
      
      expect(result.passed).toBe(true);
      
      // Verify baseline file exists
      const baselinePath = path.join(BASELINE_DIR, 'vr001-empty-editor.png');
      expect(fs.existsSync(baselinePath)).toBe(true);
    });

    test('should capture empty editor with toolbar baseline', async ({ page }) => {
      const result = await captureBaseline(page, 'vr001-empty-editor-with-toolbar', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR001: Empty editor with toolbar');
      }
      
      expect(result.passed).toBe(true);
    });
  });

  /**
   * TC-VR002: Focused editor with content baseline
   * Category: render
   * Input: Editor with sample Markdown content
   * Expected: Screenshot captured for baseline
   */
  test.describe('TC-VR002: Focused editor with content baseline', () => {
    test('should capture editor with headings baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      // Type sample Markdown content with various elements
      const sampleContent = `# Heading 1

## Heading 2

### Heading 3

This is a paragraph with **bold** and *italic* text.

- List item 1
- List item 2
- List item 3

> This is a blockquote

\`\`\`javascript
const hello = "world";
console.log(hello);
\`\`\`

[Link](https://example.com)

| Column 1 | Column 2 |
|----------|----------|
| Cell 1   | Cell 2   |
`;
      
      await page.keyboard.type(sampleContent);
      await page.waitForTimeout(1000);
      
      const result = await captureBaseline(page, 'vr002-editor-with-content', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR002: Editor with content');
      }
      
      expect(result.passed).toBe(true);
      
      // Verify baseline file exists
      const baselinePath = path.join(BASELINE_DIR, 'vr002-editor-with-content.png');
      expect(fs.existsSync(baselinePath)).toBe(true);
    });

    test('should capture editor with task list baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const taskListContent = `# Tasks

## To Do

- [ ] Task 1
- [ ] Task 2
- [x] Completed task
- [ ] Task 3

## Shopping List

- [ ] Milk
- [ ] Bread
- [ ] Eggs
`;
      
      await page.keyboard.type(taskListContent);
      await page.waitForTimeout(1000);
      
      const result = await captureBaseline(page, 'vr002-editor-with-task-list', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR002: Editor with task list');
      }
      
      expect(result.passed).toBe(true);
    });

    test('should capture editor with nested lists baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const nestedListContent = `# Nested Lists

1. First item
   - Nested bullet
   - Another nested
2. Second item
   - [ ] Nested task
   - [x] Completed nested task
3. Third item
   - Sub-item A
   - Sub-item B
`;
      
      await page.keyboard.type(nestedListContent);
      await page.waitForTimeout(1000);
      
      const result = await captureBaseline(page, 'vr002-editor-with-nested-lists', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR002: Editor with nested lists');
      }
      
      expect(result.passed).toBe(true);
    });
  });

  /**
   * TC-VR003: Dark theme baseline
   * Category: render
   * Input: Dark theme enabled
   * Expected: Screenshot captured for baseline
   */
  test.describe('TC-VR003: Dark theme baseline', () => {
    test.beforeEach(async ({ page }) => {
      // Navigate and wait for load
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      await page.waitForSelector('#editor-content', { timeout: 10000 });
      
      // Toggle to dark theme
      const themeButton = page.locator('#btn-theme');
      if (await themeButton.isVisible()) {
        await themeButton.click();
        await page.waitForTimeout(500);
      }
    });

    test('should capture dark theme empty editor baseline', async ({ page }) => {
      const result = await captureBaseline(page, 'vr003-dark-theme-empty', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR003: Dark theme empty editor');
      }
      
      expect(result.passed).toBe(true);
      
      // Verify baseline file exists
      const baselinePath = path.join(BASELINE_DIR, 'vr003-dark-theme-empty.png');
      expect(fs.existsSync(baselinePath)).toBe(true);
    });

    test('should capture dark theme editor with content baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const darkContent = `# Dark Theme Test

## Heading in Dark Mode

This paragraph should have **bold** and *italic* text.

\`\`\`
code block in dark theme
\`\`\`

> Dark blockquote
`;
      
      await page.keyboard.type(darkContent);
      await page.waitForTimeout(1000);
      
      const result = await captureBaseline(page, 'vr003-dark-theme-with-content', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR003: Dark theme with content');
      }
      
      expect(result.passed).toBe(true);
    });

    test('should capture dark theme code highlighting baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const codeContent = `# Code Highlighting

\`\`\`python
def hello():
    print("Hello, World!")
    
class MyClass:
    def __init__(self):
        self.value = 42
\`\`\`

\`\`\`rust
fn main() {
    println!("Hello, world!");
}
\`\`\`
`;
      
      await page.keyboard.type(codeContent);
      await page.waitForTimeout(1000);
      
      const result = await captureBaseline(page, 'vr003-dark-theme-code', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR003: Dark theme code');
      }
      
      expect(result.passed).toBe(true);
    });
  });

  /**
   * TC-VR004: Light theme baseline
   * Category: render
   * Input: Light theme enabled
   * Expected: Screenshot captured for baseline
   */
  test.describe('TC-VR004: Light theme baseline', () => {
    test.beforeEach(async ({ page }) => {
      // Navigate and wait for load
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      await page.waitForSelector('#editor-content', { timeout: 10000 });
      
      // Ensure light theme is active (may need to toggle if default is dark)
      const themeButton = page.locator('#btn-theme');
      if (await themeButton.isVisible()) {
        // Click to toggle to light if currently dark
        await themeButton.click();
        await page.waitForTimeout(300);
        // Click again if it toggled to dark
        const themeAttribute = await page.evaluate(() => 
          document.documentElement.getAttribute('data-theme')
        );
        if (themeAttribute === 'dark') {
          await themeButton.click();
          await page.waitForTimeout(300);
        }
      }
    });

    test('should capture light theme empty editor baseline', async ({ page }) => {
      const result = await captureBaseline(page, 'vr004-light-theme-empty', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR004: Light theme empty editor');
      }
      
      expect(result.passed).toBe(true);
      
      // Verify baseline file exists
      const baselinePath = path.join(BASELINE_DIR, 'vr004-light-theme-empty.png');
      expect(fs.existsSync(baselinePath)).toBe(true);
    });

    test('should capture light theme editor with content baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const lightContent = `# Light Theme Test

## Heading in Light Mode

This paragraph should have **bold** and *italic* text.

\`\`\`
code block in light theme
\`\`\`

> Light blockquote
`;
      
      await page.keyboard.type(lightContent);
      await page.waitForTimeout(1000);
      
      const result = await captureBaseline(page, 'vr004-light-theme-with-content', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR004: Light theme with content');
      }
      
      expect(result.passed).toBe(true);
    });

    test('should capture light theme with sidebar baseline', async ({ page }) => {
      // Open workspace first to show sidebar
      await page.keyboard.press('Control+Shift+O');
      await page.waitForTimeout(500);
      
      const result = await captureBaseline(page, 'vr004-light-theme-sidebar', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR004: Light theme with sidebar');
      }
      
      expect(result.passed).toBe(true);
    });
  });

  /**
   * TC-VR005: Focus mode baseline
   * Category: render
   * Input: Focus mode enabled
   * Expected: Screenshot captured for baseline
   */
  test.describe('TC-VR005: Focus mode baseline', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      await page.waitForSelector('#editor-content', { timeout: 10000 });
      
      // Enable focus mode
      const focusButton = page.locator('#btn-focus');
      if (await focusButton.isVisible()) {
        await focusButton.click();
        await page.waitForTimeout(500);
      }
    });

    test('should capture focus mode with content baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const focusContent = `# Focus Mode Test

## First Section

This paragraph is in focus mode.

## Second Section

Another paragraph.

## Third Section

The third paragraph.
`;
      
      await page.keyboard.type(focusContent);
      await page.waitForTimeout(1000);
      
      const result = await captureBaseline(page, 'vr005-focus-mode-content', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR005: Focus mode with content');
      }
      
      expect(result.passed).toBe(true);
      
      // Verify baseline file exists
      const baselinePath = path.join(BASELINE_DIR, 'vr005-focus-mode-content.png');
      expect(fs.existsSync(baselinePath)).toBe(true);
    });

    test('should capture focus mode with cursor on different paragraphs', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const multiParagraphContent = `# Paragraph One

First paragraph content.

# Paragraph Two

Second paragraph content.

# Paragraph Three

Third paragraph content.
`;
      
      await page.keyboard.type(multiParagraphContent);
      await page.waitForTimeout(500);
      
      // Move cursor to second paragraph
      await page.keyboard.press('ArrowUp');
      await page.keyboard.press('ArrowUp');
      await page.waitForTimeout(500);
      
      const result = await captureBaseline(page, 'vr005-focus-mode-second-paragraph', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR005: Focus mode on second paragraph');
      }
      
      expect(result.passed).toBe(true);
    });

    test('should capture focus mode full page baseline', async ({ page }) => {
      const result = await captureBaseline(page, 'vr005-focus-mode-full-page', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR005: Focus mode full page');
      }
      
      expect(result.passed).toBe(true);
    });
  });

  /**
   * TC-VR006: Typewriter mode baseline
   * Category: render
   * Input: Typewriter mode enabled
   * Expected: Screenshot captured for baseline
   */
  test.describe('TC-VR006: Typewriter mode baseline', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      await page.waitForSelector('#editor-content', { timeout: 10000 });
      
      // Enable typewriter mode
      const typewriterButton = page.locator('#btn-typewriter');
      if (await typewriterButton.isVisible()) {
        await typewriterButton.click();
        await page.waitForTimeout(500);
      }
    });

    test('should capture typewriter mode with content baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const typewriterContent = `# Typewriter Mode Test

## Section One

First section content with some text.

## Section Two

Second section with additional content.

## Section Three

Third section to test scroll behavior.

## Section Four

Fourth section for more content.

## Section Five

Fifth section to ensure cursor stays centered.
`;
      
      await page.keyboard.type(typewriterContent);
      await page.waitForTimeout(1000);
      
      const result = await captureBaseline(page, 'vr006-typewriter-mode-content', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR006: Typewriter mode with content');
      }
      
      expect(result.passed).toBe(true);
      
      // Verify baseline file exists
      const baselinePath = path.join(BASELINE_DIR, 'vr006-typewriter-mode-content.png');
      expect(fs.existsSync(baselinePath)).toBe(true);
    });

    test('should capture typewriter mode scroll behavior baseline', async ({ page }) => {
      const editor = page.locator('#editor-content');
      await editor.click();
      
      // Type multiple lines to enable scroll behavior
      const longContent = `# Long Document

${Array.from({ length: 30 }, (_, i) => `## Heading ${i + 1}

This is paragraph ${i + 1} with some content to make the document longer and test typewriter scroll behavior.`).join('\n\n')}
`;
      
      await page.keyboard.type(longContent);
      await page.waitForTimeout(1000);
      
      // Type more at the end
      await page.keyboard.type('\n\nNew content at the end.');
      await page.waitForTimeout(500);
      
      const result = await captureBaseline(page, 'vr006-typewriter-mode-scroll', { fullPage: false });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR006: Typewriter mode scroll');
      }
      
      expect(result.passed).toBe(true);
    });

    test('should capture typewriter mode full page baseline', async ({ page }) => {
      const result = await captureBaseline(page, 'vr006-typewriter-mode-full-page', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR006: Typewriter mode full page');
      }
      
      expect(result.passed).toBe(true);
    });
  });

  /**
   * TC-VR007: Export modal baseline
   * Category: render
   * Input: Export modal open
   * Expected: Screenshot captured for baseline
   */
  test.describe('TC-VR007: Export modal baseline', () => {
    test('should capture export modal baseline', async ({ page }) => {
      // Add some content first
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('# Export Test Document\n\nThis is content for export testing.');
      await page.waitForTimeout(500);
      
      // Open export modal
      const exportButton = page.locator('#btn-export');
      await exportButton.click();
      await page.waitForTimeout(500);
      
      // Verify modal is visible
      const exportModal = page.locator('text=Export Document');
      await expect(exportModal).toBeVisible();
      
      const result = await captureBaseline(page, 'vr007-export-modal', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR007: Export modal');
      }
      
      expect(result.passed).toBe(true);
      
      // Verify baseline file exists
      const baselinePath = path.join(BASELINE_DIR, 'vr007-export-modal.png');
      expect(fs.existsSync(baselinePath)).toBe(true);
    });

    test('should capture export modal with format selected baseline', async ({ page }) => {
      // Add content
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('# Format Selection Test\n\nContent for testing format selection.');
      await page.waitForTimeout(500);
      
      // Open export modal
      const exportButton = page.locator('#btn-export');
      await exportButton.click();
      await page.waitForTimeout(500);
      
      // Select PDF format
      const pdfOption = page.locator('input[type="radio"][value="pdf_a4"]');
      if (await pdfOption.isVisible()) {
        await pdfOption.click();
        await page.waitForTimeout(300);
      }
      
      const result = await captureBaseline(page, 'vr007-export-modal-pdf-selected', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR007: Export modal PDF selected');
      }
      
      expect(result.passed).toBe(true);
    });

    test('should capture export modal dark theme baseline', async ({ page }) => {
      // Toggle to dark theme
      const themeButton = page.locator('#btn-theme');
      await themeButton.click();
      await page.waitForTimeout(500);
      
      // Add content
      const editor = page.locator('#editor-content');
      await editor.click();
      
      await page.keyboard.type('# Dark Theme Export\n\nTesting export in dark mode.');
      await page.waitForTimeout(500);
      
      // Open export modal
      const exportButton = page.locator('#btn-export');
      await exportButton.click();
      await page.waitForTimeout(500);
      
      const result = await captureBaseline(page, 'vr007-export-modal-dark', { fullPage: true });
      
      if (!result.baselineExists) {
        console.log('Created new baseline for TC-VR007: Export modal dark theme');
      }
      
      expect(result.passed).toBe(true);
    });
  });

  /**
   * TC-VR008: Visual regression CI check
   * Category: integration
   * Input: Run visual regression CI
   * Expected: Passes when no visual changes detected
   */
  test.describe('TC-VR008: Visual regression CI check', () => {
    test('should have all required baseline files', async () => {
      const requiredBaselines = [
        // TC-VR001
        'vr001-empty-editor.png',
        'vr001-empty-editor-with-toolbar.png',
        // TC-VR002
        'vr002-editor-with-content.png',
        'vr002-editor-with-task-list.png',
        'vr002-editor-with-nested-lists.png',
        // TC-VR003
        'vr003-dark-theme-empty.png',
        'vr003-dark-theme-with-content.png',
        'vr003-dark-theme-code.png',
        // TC-VR004
        'vr004-light-theme-empty.png',
        'vr004-light-theme-with-content.png',
        'vr004-light-theme-sidebar.png',
        // TC-VR005
        'vr005-focus-mode-content.png',
        'vr005-focus-mode-second-paragraph.png',
        'vr005-focus-mode-full-page.png',
        // TC-VR006
        'vr006-typewriter-mode-content.png',
        'vr006-typewriter-mode-scroll.png',
        'vr006-typewriter-mode-full-page.png',
        // TC-VR007
        'vr007-export-modal.png',
        'vr007-export-modal-pdf-selected.png',
        'vr007-export-modal-dark.png',
      ];
      
      const missingBaselines = requiredBaselines.filter(
        baseline => !fs.existsSync(path.join(BASELINE_DIR, baseline))
      );
      
      // In CI, we expect all baselines to exist
      if (process.env.CI) {
        expect(missingBaselines).toHaveLength(0);
      } else {
        // In local dev, missing baselines are created automatically
        console.log(`Found ${requiredBaselines.length - missingBaselines.length} of ${requiredBaselines.length} baselines`);
        if (missingBaselines.length > 0) {
          console.log('Missing baselines (will be created on first run):', missingBaselines);
        }
      }
    });

    test('should pass visual regression when no changes detected', async ({ page }) => {
      // Navigate to app
      await page.goto('/');
      await page.waitForLoadState('domcontentloaded');
      await page.waitForSelector('#editor-content', { timeout: 10000 });
      
      // Add sample content
      const editor = page.locator('#editor-content');
      await editor.click();
      
      const testContent = '# Visual Regression Test\n\nTesting visual regression detection.';
      await page.keyboard.type(testContent);
      await page.waitForTimeout(500);
      
      // Capture current state
      const baselinePath = path.join(BASELINE_DIR, 'vr008-ci-check-test.png');
      const screenshot = await page.screenshot();
      
      if (fs.existsSync(baselinePath)) {
        // Compare with baseline
        const baseline = fs.readFileSync(baselinePath);
        const passed = compareImages(screenshot, baseline, 0.1);
        
        // Save current for review
        fs.writeFileSync(path.join(SCREENSHOTS_DIR, 'vr008-ci-check-test-current.png'), screenshot);
        
        expect(passed).toBe(true);
      } else {
        // Create baseline for CI
        fs.writeFileSync(baselinePath, screenshot);
        console.log('Created CI check baseline for vr008');
      }
    });

    test('should compare all markdown syntax baselines', async ({ page }) => {
      const syntaxTests = [
        { name: 'bold', content: '**bold text**' },
        { name: 'italic', content: '*italic text*' },
        { name: 'strikethrough', content: '~~strikethrough~~' },
        { name: 'inline-code', content: '`inline code`' },
        { name: 'link', content: '[link text](https://example.com)' },
        { name: 'image', content: '![alt text](image.png)' },
        { name: 'horizontal-rule', content: '\n---\n' },
      ];
      
      for (const test of syntaxTests) {
        // Clear editor
        await page.locator('#editor-content').click();
        await page.keyboard.press('Control+a');
        await page.keyboard.press('Backspace');
        await page.waitForTimeout(200);
        
        // Type content
        await page.keyboard.type(test.content);
        await page.waitForTimeout(500);
        
        const baselinePath = path.join(BASELINE_DIR, `vr008-syntax-${test.name}.png`);
        const screenshot = await page.screenshot({ fullPage: false });
        
        if (fs.existsSync(baselinePath)) {
          const baseline = fs.readFileSync(baselinePath);
          const passed = compareImages(screenshot, baseline, 0.1);
          fs.writeFileSync(
            path.join(SCREENSHOTS_DIR, `vr008-syntax-${test.name}-current.png`),
            screenshot
          );
          expect(passed).toBe(true);
        } else {
          fs.writeFileSync(baselinePath, screenshot);
        }
      }
    });

    test('should verify all baselines are valid images', async () => {
      const baselineFiles = fs.readdirSync(BASELINE_DIR)
        .filter(f => f.endsWith('.png'));
      
      for (const file of baselineFiles) {
        const filePath = path.join(BASELINE_DIR, file);
        const stats = fs.statSync(filePath);
        
        // Basic validation: file should be > 100 bytes (valid PNG header + data)
        expect(stats.size).toBeGreaterThan(100);
        
        // Verify PNG magic bytes
        const buffer = fs.readFileSync(filePath);
        const pngMagic = Buffer.from([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
        expect(buffer.slice(0, 8).equals(pngMagic)).toBe(true);
      }
    });
  });
});

/**
 * Additional comprehensive visual tests covering edge cases
 */
test.describe('Visual Regression - Additional Coverage', () => {
  
  test('should capture table rendering baseline', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForSelector('#editor-content', { timeout: 10000 });
    
    const editor = page.locator('#editor-content');
    await editor.click();
    
    const tableContent = `# Tables

| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |
| Cell 7   | Cell 8   | Cell 9   |
`;
    
    await page.keyboard.type(tableContent);
    await page.waitForTimeout(500);
    
    const baselinePath = path.join(BASELINE_DIR, 'additional-table-rendering.png');
    const screenshot = await page.screenshot();
    
    if (fs.existsSync(baselinePath)) {
      expect(compareImages(screenshot, fs.readFileSync(baselinePath), 0.1)).toBe(true);
    } else {
      fs.writeFileSync(baselinePath, screenshot);
    }
  });

  test('should capture frontmatter block baseline', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForSelector('#editor-content', { timeout: 10000 });
    
    const editor = page.locator('#editor-content');
    await editor.click();
    
    const frontmatterContent = `---
title: Frontmatter Document
author: Test Author
date: 2026-04-14
tags:
  - test
  - markdown
---

# Document Title

Content after frontmatter.
`;
    
    await page.keyboard.type(frontmatterContent);
    await page.waitForTimeout(500);
    
    const baselinePath = path.join(BASELINE_DIR, 'additional-frontmatter-block.png');
    const screenshot = await page.screenshot();
    
    if (fs.existsSync(baselinePath)) {
      expect(compareImages(screenshot, fs.readFileSync(baselinePath), 0.1)).toBe(true);
    } else {
      fs.writeFileSync(baselinePath, screenshot);
    }
  });

  test('should capture search panel baseline', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForSelector('#editor-content', { timeout: 10000 });
    
    // Add content with searchable text
    const editor = page.locator('#editor-content');
    await editor.click();
    
    await page.keyboard.type('# Search Test\n\nThis is a test document for search highlighting.');
    await page.waitForTimeout(500);
    
    // Open search panel
    await page.keyboard.press('Control+f');
    await page.waitForTimeout(500);
    
    const baselinePath = path.join(BASELINE_DIR, 'additional-search-panel.png');
    const screenshot = await page.screenshot();
    
    if (fs.existsSync(baselinePath)) {
      expect(compareImages(screenshot, fs.readFileSync(baselinePath), 0.1)).toBe(true);
    } else {
      fs.writeFileSync(baselinePath, screenshot);
    }
  });

  test('should capture outline panel baseline', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForSelector('#editor-content', { timeout: 10000 });
    
    // Add content with headings
    const editor = page.locator('#editor-content');
    await editor.click();
    
    const outlineContent = `# Introduction

## Background

## Problem Statement

# Methods

## Approach

## Implementation

# Results

# Conclusion
`;
    
    await page.keyboard.type(outlineContent);
    await page.waitForTimeout(500);
    
    // Open outline panel
    const outlineButton = page.locator('#btn-outline');
    await outlineButton.click();
    await page.waitForTimeout(500);
    
    const baselinePath = path.join(BASELINE_DIR, 'additional-outline-panel.png');
    const screenshot = await page.screenshot();
    
    if (fs.existsSync(baselinePath)) {
      expect(compareImages(screenshot, fs.readFileSync(baselinePath), 0.1)).toBe(true);
    } else {
      fs.writeFileSync(baselinePath, screenshot);
    }
  });

  test('should capture preferences modal baseline', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForSelector('#editor-content', { timeout: 10000 });
    
    // Open preferences modal
    const prefsButton = page.locator('#btn-preferences');
    await prefsButton.click();
    await page.waitForTimeout(500);
    
    const baselinePath = path.join(BASELINE_DIR, 'additional-preferences-modal.png');
    const screenshot = await page.screenshot();
    
    if (fs.existsSync(baselinePath)) {
      expect(compareImages(screenshot, fs.readFileSync(baselinePath), 0.1)).toBe(true);
    } else {
      fs.writeFileSync(baselinePath, screenshot);
    }
  });
});
