<template>
  <v-menu>
    <template #activator="{ on }">
      <v-btn depressed small color="primary" v-on="on">
        <v-icon left>mdi-counter</v-icon>
        Items per page:
        {{ itemsPerPage }}
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
  userModule
} from "@/plugins/store/user-module";

@Component
export default class extends Vue {
  private readonly userStore = userModule.context(this.$store);

  private readonly items = itemsPerPageData;

  get itemsPerPage(): ItemsPerPage {
    return this.userStore.getters.itemsPerPage;
  }

  setItemsPerPage(value: ItemsPerPage): void {
    this.userStore.actions.setItemsPerPage(value);
  }
}
</script>
