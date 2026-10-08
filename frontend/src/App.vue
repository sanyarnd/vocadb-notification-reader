<script setup lang="ts">
import { computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import { useLocale, useTheme } from "vuetify";

import TheAppBar from "@/components/TheAppBar.vue";
import { useSettingsStore } from "@/stores/settings";

const settings = useSettingsStore();
const theme = useTheme();
const locale = useLocale();
const route = useRoute();

const authenticated = computed(() => route.meta.requiresAuth === true);

watchEffect(() => {
  theme.change(settings.theme);
  locale.current.value = settings.locale;
  document.documentElement.lang = settings.locale;
});
</script>

<template>
  <v-app>
    <the-app-bar :authenticated="authenticated" />
    <v-main>
      <v-container fluid :class="{ 'fill-height': !authenticated }">
        <router-view />
      </v-container>
    </v-main>
  </v-app>
</template>
