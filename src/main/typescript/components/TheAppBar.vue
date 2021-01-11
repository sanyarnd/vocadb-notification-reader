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
import SwitchButtonTheme from "@/components/buttons/SwitchButtonTheme.vue";
import LogoutButton from "@/components/buttons/LogoutButton.vue";
import SwitchButtonPreferredLanguage from "@/components/buttons/SwitchButtonPreferredLanguage.vue";
import SwitchButtonLocale from "@/components/buttons/SwitchButtonLocale.vue";
import SwitchButtonItemsPerPage from "@/components/buttons/SwitchButtonItemsPerPage.vue";
import SwitchButtonPreferredService from "@/components/buttons/SwitchButtonPreferredService.vue";

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
