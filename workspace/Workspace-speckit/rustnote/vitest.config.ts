import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    include: ['www/__tests__/**/*.{test,spec}.{js,ts,jsx,tsx}'],
    exclude: ['e2e/**/*', 'node_modules/**/*'],
    coverage: {
      provider: 'v8',
      include: ['www/src/**/*.{js,ts,jsx,tsx}'],
      exclude: ['www/src/**/*.d.ts', 'www/src/**/index.{js,ts}'],
    },
  },
});
