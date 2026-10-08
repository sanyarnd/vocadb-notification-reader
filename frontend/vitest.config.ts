import { defineConfig, mergeConfig } from "vitest/config";

import viteConfig from "./vite.config";

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      environment: "jsdom",
      setupFiles: ["./tests/setup.ts"],
      include: ["tests/**/*.spec.ts"],
      server: {
        deps: {
          inline: ["vuetify"]
        }
      },
      css: false,
      coverage: {
        include: ["src/**/*.{ts,vue}"]
      }
    }
  })
);
