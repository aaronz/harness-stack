import { describe, it, expect } from 'vitest';
import { EvaluationEngine } from '../src/core/evaluationEngine';

describe('Performance Benchmark', () => {
  it('should meet <2min evaluation efficiency', async () => {
    const engine = new EvaluationEngine({
      id: 'test-config',
      provider: 'openai',
      model: 'test',
      isDefault: true
    });

    const input = {
      requirementId: 'test-001',
      content: '实现用户登录功能'
    };

    const iterations = 10;
    const start = Date.now();

    for (let i = 0; i < iterations; i++) {
      await engine.evaluate(input);
    }

    const totalTime = Date.now() - start;
    const avgTime = totalTime / iterations;

    console.log('Iterations:', iterations);
    console.log('Total time:', totalTime + 'ms');
    console.log('Average time:', avgTime.toFixed(2) + 'ms');

    expect(avgTime).toBeLessThan(120000);
  });
});
