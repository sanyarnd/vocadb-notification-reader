import { Actions, Getters, Module, Mutations } from "vuex-smart-module";
import { vuetify } from "@/plugins/vuetify";
import { PvService } from "@/backend/dto";

export type Theme = "Dark" | "Light";
export type Locale = "en" | "ja" | "ru";

export const preferredLanguageData = ["Default", "Japanese", "Romaji", "English"] as const;
export type RequestLanguage = typeof preferredLanguageData[number];

export const itemsPerPageData = [10, 25, 50, 75, 100] as const;
export type ItemsPerPage = typeof itemsPerPageData[number];

class SettingsState {
  theme: Theme = "Light";
  locale: Locale = "en";
  itemsPerPage: ItemsPerPage = 25;
  preferredLanguage: RequestLanguage = "Default";
  preferredPvService: PvService = "NicoNicoDouga";
}

class SettingsGetters extends Getters<SettingsState> {
  get preferredLanguage(): RequestLanguage {
    return this.state.preferredLanguage;
  }

  get preferredPvService(): PvService {
    return this.state.preferredPvService;
  }

  get itemsPerPage(): ItemsPerPage {
    return this.state.itemsPerPage;
  }

  get isDarkTheme(): boolean {
    return this.state.theme === "Dark";
  }

  get theme(): Theme {
    return this.state.theme;
  }

  get locale(): Locale {
    return this.state.locale;
  }
}

class SettingsMutations extends Mutations<SettingsState> {
  setPreferredLanguage(lang: RequestLanguage) {
    this.state.preferredLanguage = lang;
  }

  setPreferredPvService(service: PvService) {
    this.state.preferredPvService = service;
  }

  setItemsPerPage(itemsPerPage: ItemsPerPage) {
    this.state.itemsPerPage = itemsPerPage;
  }

  setTheme(theme: Theme): void {
    this.state.theme = theme;
    vuetify.framework.theme.dark = this.state.theme === "Dark";
  }

  setLocale(locale: Locale): void {
    this.state.locale = locale;
    vuetify.framework.lang.current = locale;
  }
}

class SettingsActions extends Actions<
  SettingsState,
  SettingsGetters,
  SettingsMutations,
  SettingsActions
> {
  setPreferredLanguage(lang: RequestLanguage) {
    this.mutations.setPreferredLanguage(lang);
  }

  setPreferredPvService(service: PvService) {
    this.mutations.setPreferredPvService(service);
  }

  setItemsPerPage(itemsPerPage: ItemsPerPage) {
    this.mutations.setItemsPerPage(itemsPerPage);
  }

  setTheme(theme: Theme) {
    this.mutations.setTheme(theme);
  }

  setLocale(locale: Locale): void {
    this.mutations.setLocale(locale);
  }
}

export const settingsModule = new Module({
  state: SettingsState,
  getters: SettingsGetters,
  mutations: SettingsMutations,
  actions: SettingsActions
});
