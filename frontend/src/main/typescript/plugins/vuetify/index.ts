import Vue from "vue";
import Vuetify from "vuetify";
import "vuetify/dist/vuetify.min.css";
import { en, ja, ru } from "@/plugins/vuetify/i18n";

Vue.use(Vuetify);

export const vuetify = new Vuetify({
  lang: {
    locales: { en, ja, ru },
    current: "en"
  },
  theme: {
    options: {
      customProperties: true
    },
    themes: {
      dark: {
        primary: "#066462",
        accent: "#FF4081",
        secondary: "#ffe18d",
        success: "#4CAF50",
        info: "#2196F3",
        warning: "#FB8C00",
        error: "#FF5252"
      },
      light: {
        primary: "#1976D2",
        accent: "#e91e63",
        secondary: "#30b1dc",
        success: "#4CAF50",
        info: "#2196F3",
        warning: "#FB8C00",
        error: "#FF5252"
      }
    }
  }
});
