import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { createHtmlPlugin } from "vite-plugin-html";

export default defineConfig(async ({ command }) => ({
  plugins: [
    react(),
    createHtmlPlugin({
      minify: command === "build",
      inject: {
        // 開発時のみ React DevTools（スタンドアロン版）に接続する
        tags:
          command === "serve"
            ? [
                {
                  injectTo: "head",
                  tag: "script",
                  attrs: { src: "http://localhost:8097" },
                },
              ]
            : [],
      },
    }),
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  root: "./src",
  publicDir: "../public",
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
}));
