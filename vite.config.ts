/// <reference types="vitest" />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  root: './src',
  build: {
    outDir: '../dist',
    minify: true,
    emptyOutDir: true,
  },
  test: {
    cache: false,
    globals: true,
    environment: 'jsdom',
    coverage: {
      enabled: true,
      reporter: ['text', 'json-summary'],
      reportsDirectory: '../coverage',
    },
  },
  plugins: [react()],
});
