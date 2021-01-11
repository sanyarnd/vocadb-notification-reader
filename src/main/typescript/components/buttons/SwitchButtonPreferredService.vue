<template>
  <v-menu>
    <template #activator="{ on }">
      <v-btn depressed small color="primary" v-on="on">
        <v-icon left>mdi-format-title</v-icon>
        {{ preferredService }}
        <v-icon right>mdi-menu-down</v-icon>
      </v-btn>
    </template>
    <v-list dense>
      <v-list-item
        v-for="(item, index) in preferredServices"
        :key="index"
        @click="setPreferredService(item)"
      >
        <v-list-item-title>{{ item }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { settingsModule } from "@/plugins/store/settings-module";
import { PvService, pvServiceData } from "@/backend/dto";

@Component
export default class extends Vue {
  private readonly userStore = settingsModule.context(this.$store);
  private readonly preferredServices = pvServiceData;

  get preferredService(): PvService {
    return this.userStore.getters.preferredPvService;
  }

  setPreferredService(value: PvService): void {
    this.userStore.actions.setPreferredPvService(value);
  }
}
</script>
