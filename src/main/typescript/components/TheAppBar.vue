<template>
  <v-app-bar app fixed color="primary">
    <div id="logo-text">Unofficial VocaDB Notification Reader</div>
    <v-spacer />
    <switch-button-preferred-service v-if="authenticated" />
    <switch-button-items-per-page v-if="authenticated" />
    <switch-button-preferred-language v-if="authenticated" />
    <switch-button-locale />
    <switch-button-theme />
    <logout-button v-if="authenticated" />
  </v-app-bar>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop } from "vue-property-decorator";
import SwitchButtonTheme from "@/components/SwitchButtonTheme.vue";
import LogoutButton from "@/components/LogoutButton.vue";
import SwitchButtonPreferredLanguage from "@/components/SwitchButtonPreferredLanguage.vue";
import SwitchButtonLocale from "@/components/SwitchButtonLocale.vue";
import SwitchButtonItemsPerPage from "@/components/SwitchButtonItemsPerPage.vue";
import SwitchButtonPreferredService from "@/components/SwitchButtonPreferredService.vue";

@Component({
  components: {
    SwitchButtonPreferredService,
    SwitchButtonItemsPerPage,
    LogoutButton,
    SwitchButtonTheme,
    SwitchButtonPreferredLanguage,
    SwitchButtonLocale
  }
})
export default class extends Vue {
  @Prop({ default: false, type: Boolean })
  private readonly authenticated!: boolean;
}
</script>

<style>
#logo-text {
  font-size: 1.5em;
  font-weight: bold;
  color: white;
}
</style>
