/// <reference types="vitest" />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { createHtmlPlugin } from 'vite-plugin-html';

export default defineConfig(({ command }) => {
  const isDev = command === 'serve';

  return {
    root: './src',
    build: {
      outDir: '../dist',
      minify: true,
      emptyOutDir: true,
    },
    test: {
      globals: true,
      environment: 'jsdom',
      coverage: {
        enabled: true,
        reporter: ['text'],
        reportsDirectory: '../coverage',
      },
    },
    plugins: [
      react(),
      createHtmlPlugin({
        minify: !isDev,
        inject: {
          data: {
            devtools: isDev
              ? `<script src="http://localhost:8097"></script>`
              : undefined,
          },
        },
      }),
    ],
  };
});
