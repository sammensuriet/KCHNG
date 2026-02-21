import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [sveltekit()],
  // Polyfill for Node.js globals in browser (needed by wallet SDK)
  define: {
    global: "globalThis",
  },
  // Properly handle stellar-sdk for browser bundling
  optimizeDeps: {
    include: ["@stellar/stellar-sdk", "buffer"],
    esbuildOptions: {
      target: "es2020",
      define: {
        global: "globalThis",
      },
    },
  },
  resolve: {
    alias: {
      buffer: "buffer",
    },
  },
  build: {
    // Increase chunk size warning limit (stellar-sdk is large)
    chunkSizeWarningLimit: 1000,
  },
});
