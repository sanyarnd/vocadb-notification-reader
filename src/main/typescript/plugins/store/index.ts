import Vue from "vue";
import Vuex from "vuex";
import { createStore, Module } from "vuex-smart-module";
import createPersistedState from "vuex-persistedstate";
import { userModule } from "@/plugins/store/user-module";
import { settingsModule } from "@/plugins/store/settings-module";
import { vuetify } from "@/plugins/vuetify";

Vue.use(Vuex);

const root = new Module({
  modules: {
    userModule: userModule,
    settingsModule: settingsModule
  }
});

export const store = createStore(root, {
  plugins: [
    createPersistedState({
      key: "cachedState"
    })
  ],
  strict: process.env.NODE_ENV !== "production"
});

export function vuetifyRestoreHook() {
  const settingsStore = settingsModule.context(store);
  vuetify.framework.lang.current = settingsStore.getters.locale;
  vuetify.framework.theme.dark = settingsStore.getters.isDarkTheme;
}
