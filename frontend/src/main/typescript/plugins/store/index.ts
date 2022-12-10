import Vue from "vue";
import Vuex from "vuex";
import { createStore, Module } from "vuex-smart-module";
import createPersistedState from "vuex-persistedstate";
import { accountModule } from "@/plugins/store/account-module";
import { settingsModule } from "@/plugins/store/settings-module";
import { vuetify } from "@/plugins/vuetify";

Vue.use(Vuex);

const root = new Module({
  modules: {
    accountModule: accountModule,
    settingsModule: settingsModule
  }
});

export const store = createStore(root, {
  plugins: [createPersistedState({ key: "state" })],
  strict: process.env.NODE_ENV !== "production"
});

export function vuetifyRestoreHook(): void {
  const settingsStore = settingsModule.context(store);
  vuetify.framework.lang.current = settingsStore.getters.locale;
  vuetify.framework.theme.dark = settingsStore.getters.isDarkTheme;
}
