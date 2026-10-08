import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";

import { createVuetify } from "vuetify";
import { aliases, mdi } from "vuetify/iconsets/mdi";

import { messages } from "@/i18n";

export function createAppVuetify() {
  return createVuetify({
    icons: { defaultSet: "mdi", aliases, sets: { mdi } },
    locale: { locale: "en", fallback: "en", messages },
    theme: {
      defaultTheme: "light",
      themes: {
        light: {
          dark: false,
          colors: {
            primary: "#1976D2",
            secondary: "#30b1dc",
            accent: "#e91e63",
            success: "#4CAF50",
            info: "#2196F3",
            warning: "#FB8C00",
            error: "#FF5252"
          }
        },
        dark: {
          dark: true,
          colors: {
            primary: "#066462",
            secondary: "#ffe18d",
            accent: "#FF4081",
            success: "#4CAF50",
            info: "#2196F3",
            warning: "#FB8C00",
            error: "#FF5252"
          }
        }
      }
    }
  });
}
