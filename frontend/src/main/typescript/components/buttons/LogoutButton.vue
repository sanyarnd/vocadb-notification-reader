<template>
  <v-btn @click="logout">
    {{ $vuetify.lang.t("$vuetify.logout") }}
    <v-icon right>mdi-logout</v-icon>
  </v-btn>
</template>

<script lang="ts">
import Vue from "vue";
import { Component } from "vue-property-decorator";
import { accountModule } from "@/plugins/store/account-module";
import SwitchButtonTheme from "@/components/buttons/SwitchButtonTheme.vue";

@Component({ components: { SwitchButtonTheme } })
export default class extends Vue {
  private accountStore = accountModule.context(this.$store);

  async logout(): Promise<void> {
    await this.accountStore.actions.logout();
  }
}
</script>
