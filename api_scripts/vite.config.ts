import { resolve } from "path";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    // Tauri supports es2021
    target: ["es2021", "chrome100", "safari11"],
    lib: {
      entry: resolve(__dirname, "src/index.ts"),
      name: "@holochain/launcher-api-scripts",
    },
  },
});
