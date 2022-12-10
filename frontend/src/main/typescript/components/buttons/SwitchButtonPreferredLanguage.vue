<template>
  <v-menu>
    <template #activator="{ on }">
      <v-btn depressed small color="primary" v-on="on">
        <v-icon left>mdi-format-title</v-icon>
        {{ preferredLanguages.get(preferredLanguage) }}
        <v-icon right>mdi-menu-down</v-icon>
      </v-btn>
    </template>
    <v-list dense>
      <v-list-item
        v-for="(item, index) in preferredLanguages"
        :key="index"
        @click="setPreferredLanguage(item[0])"
      >
        <v-list-item-title>{{ item[1] }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import {
  RequestLanguage,
  settingsModule
} from "@/plugins/store/settings-module";

@Component
export default class extends Vue {
  private readonly settingsStore = settingsModule.context(this.$store);
  private readonly preferredLanguages: Map<RequestLanguage, string> = new Map([
    ["Default", this.$vuetify.lang.t("$vuetify.preferredLanguage.original")],
    ["Romaji", this.$vuetify.lang.t("$vuetify.preferredLanguage.romanized")],
    ["English", this.$vuetify.lang.t("$vuetify.preferredLanguage.english")],
    ["Japanese", this.$vuetify.lang.t("$vuetify.preferredLanguage.nonEnglish")]
  ]);

  get preferredLanguage(): RequestLanguage {
    return this.settingsStore.getters.preferredLanguage;
  }

  setPreferredLanguage(value: RequestLanguage): void {
    this.settingsStore.actions.setPreferredLanguage(value);
  }
}
</script>
