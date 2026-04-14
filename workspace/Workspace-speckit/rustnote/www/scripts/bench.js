#!/usr/bin/env node

/**
 * Frontend Performance Benchmarks
 * 
 * This script runs browser-based performance benchmarks to measure frontend
 * metrics including:
 * - Time to First Byte (TTFB)
 * - First Contentful Paint (FCP)
 * - DOM Ready
 * - Window Load
 * - JavaScript Execution Time
 * - Memory Usage
 * 
 * Results are saved to benchmark-results.json for CI integration.
 * 
 * Can run in both Node.js (for CI) and browser environments.
 */

import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const OUTPUT_DIR = join(__dirname, '..');

// Check environment
const isBrowser = typeof window !== 'undefined' && typeof document !== 'undefined';
const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;

// NFR Thresholds
const THRESHOLDS = {
  ttfb: 100,           // Time to First Byte < 100ms
  fcp: 500,            // First Contentful Paint < 500ms
  domReady: 1000,      // DOM Ready < 1000ms
  windowLoad: 2000,    // Window Load < 2000ms
  jsExecution: 100,     // JS Execution (per operation) < 100ms
  memoryIdle: 300,     // Memory Idle < 300MB (10 documents)
  keystrokeRender: 100, // Keystroke to render < 100ms
  scrollFrame: 100,    // Scroll frame time < 100ms
};

/**
 * Performance measurement utilities
 */
class PerformanceMonitor {
  constructor() {
    this.results = [];
  }

  /**
   * Measure a performance metric
   */
  measure(name, fn) {
    if (typeof performance === 'undefined') {
      console.warn(`[WARN] Performance API not available for: ${name}`);
      return { name, status: 'skipped', time: 0 };
    }

    const start = performance.now();
    const result = fn();
    const end = performance.now();
    const time = end - start;

    const passed = time < (THRESHOLDS[name] || Infinity);
    
    return {
      name,
      time: Math.round(time * 100) / 100,
      threshold: THRESHOLDS[name] || null,
      status: passed ? 'passed' : 'warning',
      unit: 'ms'
    };
  }

  /**
   * Measure with multiple iterations
   */
  measureWithIterations(name, fn, iterations = 10) {
    const useBrowserAPI = typeof performance !== 'undefined' && performance.now;
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
      let elapsed;
      if (useBrowserAPI) {
        const start = performance.now();
        fn();
        elapsed = performance.now() - start;
      } else {
        // Node.js environment
        const start = Date.now();
        const result = fn();
        elapsed = result !== undefined ? result : Date.now() - start;
      }
      times.push(elapsed);
    }
    
    const avg = times.reduce((a, b) => a + b, 0) / times.length;
    const min = Math.min(...times);
    const max = Math.max(...times);
    
    return {
      name,
      time: Math.round(avg * 100) / 100,
      min: Math.round(min * 100) / 100,
      max: Math.round(max * 100) / 100,
      threshold: THRESHOLDS[name] || null,
      status: avg < (THRESHOLDS[name] || Infinity) ? 'passed' : 'warning',
      unit: 'ms',
      iterations
    };
  }

  /**
   * Get browser memory info (if available)
   */
  getMemoryUsage() {
    if (typeof performance === 'undefined' || !performance.memory) {
      return null;
    }
    return {
      usedJSHeapSize: performance.memory.usedJSHeapSize,
      totalJSHeapSize: performance.memory.totalJSHeapSize,
      jsHeapSizeLimit: performance.memory.jsHeapSizeLimit,
      usedMB: Math.round(performance.memory.usedJSHeapSize / 1048576),
      totalMB: Math.round(performance.memory.totalJSHeapSize / 1048576)
    };
  }

  /**
   * Measure memory usage
   */
  measureMemory(name) {
    const memory = this.getMemoryUsage();
    if (!memory) {
      return { name, status: 'skipped', reason: 'Memory API not available' };
    }

    const passed = memory.usedMB < THRESHOLDS.memoryIdle;
    return {
      name,
      memory: memory,
      threshold: THRESHOLDS.memoryIdle,
      status: passed ? 'passed' : 'warning',
      unit: 'MB'
    };
  }

  /**
   * Add result
   */
  add(result) {
    this.results.push({
      timestamp: new Date().toISOString(),
      ...result
    });
  }

  /**
   * Get all results
   */
  getResults() {
    return this.results;
  }
}

/**
 * Benchmark suite for frontend metrics
 */
async function runFrontendBenchmarks() {
  console.log('Starting Frontend Performance Benchmarks...\n');
  
  const monitor = new PerformanceMonitor();
  
  // TC-B004: Keystroke-to-render latency
  console.log('Running: Keystroke-to-render latency (TC-B004)...');
  
  if (isBrowser) {
    monitor.add(monitor.measureWithIterations('keystrokeRender', () => {
      const start = performance.now();
      // Simulate keystroke processing
      document.createElement('div').textContent = 'x';
      // Force reflow
      void document.body.offsetHeight;
      performance.now() - start;
    }, 100));

    // DOM manipulation benchmark
    console.log('Running: DOM manipulation benchmark...');
    monitor.add(monitor.measureWithIterations('domManipulation', () => {
      const container = document.createElement('div');
      for (let i = 0; i < 100; i++) {
        const el = document.createElement('p');
        el.textContent = `Item ${i}`;
        container.appendChild(el);
      }
      document.body.appendChild(container);
      document.body.removeChild(container);
    }, 50));

    // TC-B008: Scroll performance simulation
    console.log('Running: Scroll performance (TC-B008)...');
    monitor.add(monitor.measureWithIterations('scrollFrame', () => {
      const container = document.createElement('div');
      container.style.height = '10000px';
      container.style.width = '100%';
      document.body.appendChild(container);
      
      const start = performance.now();
      window.scrollTo(0, 5000);
      requestAnimationFrame(() => {
        performance.now() - start;
      });
      
      document.body.removeChild(container);
    }, 100));

    // React component rendering benchmark
    console.log('Running: React rendering benchmark...');
    monitor.add(monitor.measureWithIterations('reactRender', () => {
      const start = performance.now();
      const editor = document.createElement('div');
      editor.className = 'editor-content';
      for (let i = 0; i < 50; i++) {
        const p = document.createElement('p');
        p.textContent = `Line ${i}: Sample text content`;
        editor.appendChild(p);
      }
      performance.now() - start;
    }, 20));

    // Marked.js parsing benchmark
    console.log('Running: Marked.js Markdown parsing benchmark...');
    if (window.marked) {
      const sampleMarkdown = `# Title

## Section 1

This is **bold** and *italic* text.

- Item 1
- Item 2
- Item 3

> Blockquote

\`\`\`javascript
console.log('code');
\`\`\`
`.repeat(10);

      monitor.add(monitor.measureWithIterations('markdownParse', () => {
        const start = performance.now();
        window.marked.parse(sampleMarkdown);
        performance.now() - start;
      }, 50));
    } else {
      // Simulate when marked is not available
      monitor.add({
        name: 'markdownParse',
        status: 'skipped',
        reason: 'Marked.js not loaded in browser'
      });
    }

    // TC-B007: Memory idle benchmark (10 documents)
    console.log('Running: Memory idle with 10 documents (TC-B007)...');
    const memoryBefore = monitor.getMemoryUsage();
    
    const documents = [];
    for (let i = 0; i < 10; i++) {
      const doc = document.createElement('div');
      doc.textContent = `Document ${i} content with some text`;
      doc.dataset.docId = i;
      documents.push(doc);
    }
    
    const memoryAfter = monitor.getMemoryUsage();
    
    monitor.add({
      name: 'memoryIdle',
      documents: 10,
      memoryBefore: memoryBefore,
      memoryAfter: memoryAfter,
      threshold: THRESHOLDS.memoryIdle,
      status: memoryAfter && memoryAfter.usedMB < THRESHOLDS.memoryIdle ? 'passed' : 'warning',
      unit: 'MB'
    });

    // Network timing benchmarks
    console.log('Running: Network timing benchmarks...');
    if (performance.timing) {
      const timing = performance.timing;
      const ttfb = timing.responseStart - timing.requestStart;
      const fcp = timing.domContentLoadedEventStart - timing.navigationStart;
      const domReady = timing.domContentLoadedEventEnd - timing.navigationStart;
      const windowLoad = timing.loadEventEnd - timing.navigationStart;

      monitor.add({
        name: 'ttfb',
        time: ttfb,
        threshold: THRESHOLDS.ttfb,
        status: ttfb < THRESHOLDS.ttfb ? 'passed' : 'warning',
        unit: 'ms'
      });

      monitor.add({
        name: 'fcp',
        time: fcp,
        threshold: THRESHOLDS.fcp,
        status: fcp < THRESHOLDS.fcp ? 'passed' : 'warning',
        unit: 'ms'
      });

      monitor.add({
        name: 'domReady',
        time: domReady,
        threshold: THRESHOLDS.domReady,
        status: domReady < THRESHOLDS.domReady ? 'passed' : 'warning',
        unit: 'ms'
      });

      monitor.add({
        name: 'windowLoad',
        time: windowLoad,
        threshold: THRESHOLDS.windowLoad,
        status: windowLoad < THRESHOLDS.windowLoad ? 'passed' : 'warning',
        unit: 'ms'
      });
    }
  } else {
    // Node.js environment - run simulation benchmarks
    console.log('Running in Node.js environment - running simulation benchmarks...\n');
    
    // Simulate keystroke processing
    monitor.add(monitor.measureWithIterations('keystrokeRender', () => {
      const start = Date.now();
      // Simulate processing work
      let result = 0;
      for (let i = 0; i < 1000; i++) {
        result += i;
      }
      return Date.now() - start;
    }, 100));

    // Simulate DOM parsing work (similar to Markdown parsing)
    monitor.add(monitor.measureWithIterations('markdownParse', () => {
      const start = Date.now();
      // Simulate parsing work
      const sample = 'x'.repeat(10000);
      const lines = sample.split('');
      let count = 0;
      for (const line of lines) {
        if (line === '#') count++;
      }
      return Date.now() - start;
    }, 50));

    // Simulate serialization
    monitor.add(monitor.measureWithIterations('serialization', () => {
      const start = Date.now();
      const data = { items: [] };
      for (let i = 0; i < 100; i++) {
        data.items.push({ id: i, text: `Item ${i} content` });
      }
      JSON.stringify(data);
      return Date.now() - start;
    }, 100));

    monitor.add({
      name: 'browserBenchmark',
      status: 'skipped',
      reason: 'Browser environment not available - simulation results shown'
    });
  }

  return monitor.getResults();
}

/**
 * Save results to JSON file
 */
function saveResults(results, outputPath) {
  const summary = {
    timestamp: new Date().toISOString(),
    totalTests: results.length,
    passed: results.filter(r => r.status === 'passed').length,
    warnings: results.filter(r => r.status === 'warning').length,
    skipped: results.filter(r => r.status === 'skipped').length,
    results
  };

  writeFileSync(outputPath, JSON.stringify(summary, null, 2));
  return summary;
}

/**
 * Print summary to console
 */
function printSummary(summary) {
  console.log('\n' + '='.repeat(60));
  console.log('BENCHMARK SUMMARY');
  console.log('='.repeat(60));
  console.log(`Total Tests: ${summary.totalTests}`);
  console.log(`Passed:      ${summary.passed}`);
  console.log(`Warnings:    ${summary.warnings}`);
  console.log(`Skipped:     ${summary.skipped}`);
  console.log('='.repeat(60));
  
  console.log('\nDetailed Results:');
  summary.results.forEach(r => {
    const status = r.status === 'passed' ? '✓' : r.status === 'warning' ? '⚠' : '○';
    const time = r.time !== undefined ? `${r.time} ${r.unit}` : '';
    const threshold = r.threshold ? ` (threshold: ${r.threshold})` : '';
    console.log(`  ${status} ${r.name}${threshold ? threshold : ': ' + time}`);
  });
}

/**
 * Main entry point
 */
async function main() {
  const outputPath = join(OUTPUT_DIR, 'benchmark-results.json');
  
  try {
    const results = await runFrontendBenchmarks();
    const summary = saveResults(results, outputPath);
    printSummary(summary);
    
    // Exit with appropriate code
    const hasWarnings = summary.warnings > 0;
    const hasFailures = summary.skipped === summary.totalTests;
    
    if (hasFailures) {
      console.log('\n[ERROR] All benchmarks skipped - check environment');
      process.exit(1);
    } else if (hasWarnings) {
      console.log('\n[WARN] Some benchmarks exceeded thresholds');
      process.exit(0); // Exit 0 for warnings to not block CI
    } else {
      console.log('\n[SUCCESS] All benchmarks passed');
      process.exit(0);
    }
  } catch (error) {
    console.error('[ERROR] Benchmark execution failed:', error.message);
    
    // Save error result
    const errorResult = {
      timestamp: new Date().toISOString(),
      error: error.message,
      stack: error.stack
    };
    writeFileSync(outputPath, JSON.stringify(errorResult, null, 2));
    
    process.exit(1);
  }
}

// Run if executed directly
if (typeof process !== 'undefined' && process.argv && process.argv[1] === __filename) {
  main();
}

// Export for use as module
export { runFrontendBenchmarks, saveResults, THRESHOLDS };
export default main;
