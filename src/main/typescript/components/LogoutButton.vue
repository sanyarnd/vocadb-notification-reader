<template>
  <v-btn v-if="isAuthenticated" @click="logout">
    {{ $vuetify.lang.t("$vuetify.logout") }}
    <v-icon right>mdi-logout</v-icon>
  </v-btn>
</template>

<script lang="ts">
import Vue from "vue";
import { Component } from "vue-property-decorator";
import { userModule } from "@/plugins/store/user-module";
import SwitchButtonTheme from "@/components/SwitchButtonTheme.vue";

@Component({ components: { SwitchButtonTheme } })
export default class extends Vue {
  private userStore = userModule.context(this.$store);

  get isAuthenticated(): boolean {
    return this.userStore.getters.isAuthenticated;
  }

  async logout(): Promise<void> {
    await this.userStore.actions.logout();
  }
}
</script>
