import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [sveltekit()],
  // Properly handle stellar-sdk for browser bundling
  optimizeDeps: {
    include: ["@stellar/stellar-sdk"],
    esbuildOptions: {
      target: "es2020",
    },
  },
  build: {
    // Increase chunk size warning limit (stellar-sdk is large)
    chunkSizeWarningLimit: 1000,
  },
});
