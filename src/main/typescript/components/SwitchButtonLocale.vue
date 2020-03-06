<template>
  <v-menu>
    <template v-slot:activator="{ on }">
      <v-btn depressed small color="primary" v-on="on">
        <v-icon left>mdi-translate</v-icon>
        {{ locale.text }}
        <v-icon right>mdi-menu-down</v-icon>
      </v-btn>
    </template>
    <v-list dense>
      <v-list-item
        v-for="(item, index) in locales"
        :key="index"
        @click="setLocale(item.code)"
      >
        <v-list-item-title>{{ item.text }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { LocaleCode, settingsModule } from "@/plugins/store/settings-module";

@Component
export default class extends Vue {
  private readonly settingsStore = settingsModule.context(this.$store);

  private readonly locales: Array<Locale> = [
    { code: "en", text: "English" },
    { code: "ja", text: "日本語" },
    { code: "ru", text: "Русский" }
  ];

  get locale(): Locale {
    const locale = this.settingsStore.getters.locale;
    const find = this.locales.find(x => x.code === locale);
    return find === undefined ? this.locales?.[0] : find;
  }

  setLocale(locale: LocaleCode) {
    this.settingsStore.actions.setLocale(locale);
  }
}

interface Locale {
  code: LocaleCode;
  text: string;
}
</script>
