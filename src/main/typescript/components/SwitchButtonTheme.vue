<template>
  <v-btn icon @click="darkTheme = !darkTheme">
    <v-icon v-if="darkTheme">mdi-white-balance-sunny</v-icon>
    <v-icon v-else>mdi-weather-night</v-icon>
  </v-btn>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { settingsModule } from "@/plugins/store/settings-module";

@Component
export default class extends Vue {
  private settingsStore = settingsModule.context(this.$store);

  get darkTheme() {
    return this.settingsStore.getters.isDarkTheme;
  }

  set darkTheme(value: boolean) {
    this.settingsStore.actions.setTheme(value ? "Dark" : "Light");
  }
}
</script>
