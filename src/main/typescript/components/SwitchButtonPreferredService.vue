<template>
  <v-menu>
    <template v-slot:activator="{ on }">
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
import { userModule } from "@/plugins/store/user-module";
import { PvService, pvServiceData } from "@/backend/dto";

@Component
export default class extends Vue {
  private readonly userStore = userModule.context(this.$store);
  private readonly preferredServices = pvServiceData;

  get preferredService(): PvService {
    return this.userStore.getters.preferredService;
  }

  setPreferredService(value: PvService) {
    this.userStore.actions.setPreferredService(value);
  }
}
</script>
