import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

export default defineConfig({
  root: './src',
  build: {
    outDir: '../dist',
    minify: true,
    emptyOutDir: true,
  },
  plugins: [preact()],
});
