import { defineConfig } from "vite";

import react from "@vitejs/plugin-react";
import { createHtmlPlugin } from "vite-plugin-html";

export default defineConfig(async ({ command }) => ({
  plugins: [
    react(),
    createHtmlPlugin({
      minify: command === "build",
      inject: {
        data: {
          devtools:
            command === "serve"
              ? `<script src="http://localhost:8097"></script>`
              : undefined,
        },
      },
    }),
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  root: "./src",
  publicDir: "../public",
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
}));
