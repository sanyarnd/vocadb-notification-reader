<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { useLocale } from "vuetify";

import { pvServices, requestLanguages } from "@/api/dto";
import SettingMenu from "@/components/SettingMenu.vue";
import { useAccountStore } from "@/stores/account";
import { itemsPerPageOptions, type Locale, useSettingsStore } from "@/stores/settings";

defineProps<{ authenticated: boolean }>();

const settings = useSettingsStore();
const account = useAccountStore();
const router = useRouter();
const { t } = useLocale();

const localeNames: Record<Locale, string> = { en: "English", ja: "日本語", ru: "Русский" };
const localeItems = Object.entries(localeNames).map(([value, title]) => ({
  value: value as Locale,
  title
}));
const serviceItems = pvServices.map(value => ({ value, title: value }));
const itemsPerPageItems = itemsPerPageOptions.map(value => ({ value, title: String(value) }));
const languageItems = computed(() =>
  requestLanguages.map(value => ({ value, title: t(`$vuetify.preferredLanguage.${value}`) }))
);

async function logout(): Promise<void> {
  account.logout();
  await router.push({ name: "login" });
}
</script>

<template>
  <v-app-bar color="primary" density="comfortable">
    <v-app-bar-title class="font-weight-bold"
      >Unofficial VocaDB Notification Reader</v-app-bar-title
    >

    <template v-if="authenticated">
      <setting-menu
        v-model="settings.preferredPvService"
        icon="mdi-play-box-outline"
        :label="settings.preferredPvService"
        :items="serviceItems"
      />
      <setting-menu
        v-model="settings.itemsPerPage"
        icon="mdi-counter"
        :label="`${t('$vuetify.buttons.itemsPerPage')}: ${settings.itemsPerPage}`"
        :items="itemsPerPageItems"
      />
      <setting-menu
        v-model="settings.preferredLanguage"
        icon="mdi-format-title"
        :label="t(`$vuetify.preferredLanguage.${settings.preferredLanguage}`)"
        :items="languageItems"
      />
    </template>
    <setting-menu
      v-model="settings.locale"
      icon="mdi-translate"
      :label="localeNames[settings.locale]"
      :items="localeItems"
    />
    <v-btn
      :icon="settings.theme === 'dark' ? 'mdi-white-balance-sunny' : 'mdi-weather-night'"
      :aria-label="t('$vuetify.buttons.theme')"
      @click="settings.toggleTheme()"
    />
    <v-btn v-if="authenticated" append-icon="mdi-logout" @click="logout">
      {{ t("$vuetify.logout") }}
    </v-btn>
  </v-app-bar>
</template>
