import { Actions, Getters, Module, Mutations } from "vuex-smart-module";
import { vuetify } from "@/plugins/vuetify";

export type Theme = "Dark" | "Light";
export type LocaleCode = "en" | "ja" | "ru";

class SettingsState {
  theme: Theme = "Light";
  locale: LocaleCode = "en";
}

class SettingsGetters extends Getters<SettingsState> {
  get isDarkTheme(): boolean {
    return this.state.theme === "Dark";
  }

  get theme(): Theme {
    return this.state.theme;
  }

  get locale(): LocaleCode {
    return this.state.locale;
  }
}

class SettingsMutations extends Mutations<SettingsState> {
  setTheme(theme: Theme): void {
    this.state.theme = theme;
    vuetify.framework.theme.dark = this.state.theme === "Dark";
  }

  setLocale(locale: LocaleCode): void {
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
  setTheme(theme: Theme) {
    this.mutations.setTheme(theme);
  }

  setLocale(locale: LocaleCode): void {
    this.mutations.setLocale(locale);
  }
}

export const settingsModule = new Module({
  state: SettingsState,
  getters: SettingsGetters,
  mutations: SettingsMutations,
  actions: SettingsActions
});
