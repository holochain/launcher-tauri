import { resolve } from "path";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    lib: {
      entry: resolve(__dirname, "src/zomeCallSigning.ts"),
      name: "ZomeCallSigning",
    },
    // Tauri supports es2021
    target: ["es2021", "chrome100", "safari11"],
    outDir: "dist",
    // don't minify for debug builds
    minify: "esbuild",
  },
});
