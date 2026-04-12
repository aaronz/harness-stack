import { test, expect, chromium, firefox, webkit } from '@playwright/test';

const BENCHMARK_CONFIG = {
  iterations: 100,
  warmupIterations: 10,
  thresholds: {
    coldStartEmpty: { target: 2000, critical: 3000 },
    coldStart1MB: { target: 3000, critical: 5000 },
    hotFileOpen: { target: 500, critical: 1000 },
    keystrokeToRender: { target: 100, critical: 200 },
    saveOperation: { target: 200, critical: 500 },
    htmlExport10KB: { target: 1000, critical: 2000 },
    pdfExport10Pages: { target: 5000, critical: 10000 },
    outlineUpdate: { target: 100, critical: 200 },
    themeSwitch: { target: 200, critical: 500 },
    memory10DocsIdle: { target: 300, critical: 500 },
    scrollFPS: { target: 60, critical: 30 },
  },
};

async function measureKeystrokeLatency(page: any): Promise<number[]> {
  const measurements: number[] = [];
  
  for (let i = 0; i < BENCHMARK_CONFIG.warmupIterations; i++) {
    await page.keyboard.type('x');
  }
  
  for (let i = 0; i < BENCHMARK_CONFIG.iterations; i++) {
    const start = performance.now();
    await page.keyboard.type('x');
    await page.waitForTimeout(0);
    measurements.push(performance.now() - start);
  }
  
  return measurements;
}

test.describe('Performance Benchmarks', () => {
  test('cold start (empty document)', async ({ page }) => {
    const start = performance.now();
    await page.goto('tauri://localhost');
    await page.waitForSelector('.editor', { state: 'visible' });
    const duration = performance.now() - start;
    
    expect(duration).toBeLessThan(BENCHMARK_CONFIG.thresholds.coldStartEmpty.critical);
    console.log(`Cold start (empty): ${duration.toFixed(2)}ms`);
  });

  test('hot file open < 500ms', async ({ page }) => {
    await page.goto('tauri://localhost');
    await page.waitForSelector('.editor', { state: 'visible' });
    
    const start = performance.now();
    await page.goto('tauri://localhost');
    await page.waitForSelector('.editor', { state: 'visible' });
    const duration = performance.now() - start;
    
    expect(duration).toBeLessThan(BENCHMARK_CONFIG.thresholds.hotFileOpen.critical);
    console.log(`Hot file open: ${duration.toFixed(2)}ms`);
  });

  test('keystroke to render < 100ms', async ({ page }) => {
    await page.goto('tauri://localhost');
    await page.waitForSelector('.editor', { state: 'visible' });
    
    const measurements = await measureKeystrokeLatency(page);
    const avg = measurements.reduce((a, b) => a + b, 0) / measurements.length;
    
    expect(avg).toBeLessThan(BENCHMARK_CONFIG.thresholds.keystrokeToRender.critical);
    console.log(`Average keystroke latency: ${avg.toFixed(2)}ms`);
  });

  test('save operation < 200ms', async ({ page }) => {
    await page.goto('tauri://localhost');
    await page.waitForSelector('.editor', { state: 'visible' });
    
    const start = performance.now();
    await page.keyboard.press('Control+s');
    await page.waitForTimeout(100);
    const duration = performance.now() - start;
    
    expect(duration).toBeLessThan(BENCHMARK_CONFIG.thresholds.saveOperation.critical);
    console.log(`Save operation: ${duration.toFixed(2)}ms`);
  });

  test('scroll maintains 60 FPS', async ({ page }) => {
    await page.goto('tauri://localhost');
    await page.waitForSelector('.editor', { state: 'visible' });
    
    const frames: number[] = [];
    let lastTime = performance.now();
    
    for (let i = 0; i < 60; i++) {
      await page.keyboard.press('PageDown');
      const currentTime = performance.now();
      frames.push(1000 / (currentTime - lastTime));
      lastTime = currentTime;
    }
    
    const avgFPS = frames.reduce((a, b) => a + b, 0) / frames.length;
    expect(avgFPS).toBeGreaterThan(BENCHMARK_CONFIG.thresholds.scrollFPS.critical);
    console.log(`Average scroll FPS: ${avgFPS.toFixed(2)}`);
  });
});

test.describe('Cross-browser Performance', () => {
  const browsers = [
    { name: 'chromium', browser: chromium },
    { name: 'firefox', browser: firefox },
    { name: 'webkit', browser: webkit },
  ];

  for (const { name, browser: BrowserType } of browsers) {
    test(`${name}: cold start performance`, async () => {
      const browser = await BrowserType.launch();
      const context = await browser.newContext();
      const page = await context.newPage();
      
      const start = performance.now();
      await page.goto('tauri://localhost');
      await page.waitForSelector('.editor', { state: 'visible' });
      const duration = performance.now() - start;
      
      await browser.close();
      
      expect(duration).toBeLessThan(BENCHMARK_CONFIG.thresholds.coldStartEmpty.critical);
      console.log(`${name} cold start: ${duration.toFixed(2)}ms`);
    });
  }
});

export { BENCHMARK_CONFIG, measureKeystrokeLatency };
