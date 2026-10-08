import { fileURLToPath, URL } from "node:url";

import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";
import vuetify from "vite-plugin-vuetify";

export default defineConfig({
  // Relative asset paths let the bundle be served from any CDN prefix.
  base: "./",
  plugins: [vue(), vuetify({ autoImport: true })],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url))
    }
  },
  server: {
    proxy: {
      "/api": "http://localhost:8080"
    }
  }
});
