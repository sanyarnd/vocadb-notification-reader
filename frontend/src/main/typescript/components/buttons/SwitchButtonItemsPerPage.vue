<template>
  <v-menu>
    <template #activator="{ on }">
      <v-btn depressed small color="primary" v-on="on">
        <v-icon left>mdi-counter</v-icon>
        {{
          $vuetify.lang.t("$vuetify.buttons.itemsPerPage") + ": " + itemsPerPage
        }}
        <v-icon right>mdi-menu-down</v-icon>
      </v-btn>
    </template>
    <v-list dense>
      <v-list-item
        v-for="(item, index) in items"
        :key="index"
        @click="setItemsPerPage(item)"
      >
        <v-list-item-title>{{ item }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import {
  ItemsPerPage,
  itemsPerPageData,
  settingsModule
} from "@/plugins/store/settings-module";

@Component
export default class extends Vue {
  private readonly settingsStore = settingsModule.context(this.$store);
  private readonly items = itemsPerPageData;

  get itemsPerPage(): ItemsPerPage {
    return this.settingsStore.getters.itemsPerPage;
  }

  setItemsPerPage(value: ItemsPerPage): void {
    this.settingsStore.actions.setItemsPerPage(value);
  }
}
</script>
