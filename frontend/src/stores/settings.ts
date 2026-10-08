import { useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";

import type { PvService, RequestLanguage } from "@/api/dto";

export const themes = ["light", "dark"] as const;
export type Theme = (typeof themes)[number];

export const locales = ["en", "ja", "ru"] as const;
export type Locale = (typeof locales)[number];

export const itemsPerPageOptions = [10, 25, 50, 75, 100] as const;
export type ItemsPerPage = (typeof itemsPerPageOptions)[number];

export const useSettingsStore = defineStore("settings", () => {
  const theme = useLocalStorage<Theme>("settings.theme", "light");
  const locale = useLocalStorage<Locale>("settings.locale", "en");
  const itemsPerPage = useLocalStorage<ItemsPerPage>("settings.itemsPerPage", 25);
  const preferredLanguage = useLocalStorage<RequestLanguage>(
    "settings.preferredLanguage",
    "Default"
  );
  const preferredPvService = useLocalStorage<PvService>(
    "settings.preferredPvService",
    "NicoNicoDouga"
  );

  function toggleTheme(): void {
    theme.value = theme.value === "dark" ? "light" : "dark";
  }

  return { theme, locale, itemsPerPage, preferredLanguage, preferredPvService, toggleTheme };
});
