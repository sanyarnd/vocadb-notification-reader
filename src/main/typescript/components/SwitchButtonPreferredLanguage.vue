<template>
  <v-menu>
    <template v-slot:activator="{ on }">
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
import { PreferredLanguage, userModule } from "@/plugins/store/user-module";

@Component
export default class extends Vue {
  private readonly userStore = userModule.context(this.$store);
  private readonly preferredLanguages: Map<PreferredLanguage, string> = new Map(
    [
      ["DEFAULT", this.$vuetify.lang.t("$vuetify.preferredLanguage.original")],
      ["ROMAJI", this.$vuetify.lang.t("$vuetify.preferredLanguage.romanized")],
      ["ENGLISH", this.$vuetify.lang.t("$vuetify.preferredLanguage.english")],
      [
        "JAPANESE",
        this.$vuetify.lang.t("$vuetify.preferredLanguage.nonEnglish")
      ]
    ]
  );

  get preferredLanguage(): PreferredLanguage {
    return this.userStore.getters.preferredLanguage;
  }

  setPreferredLanguage(value: PreferredLanguage) {
    this.userStore.actions.setPreferredLanguage(value);
  }
}
</script>
