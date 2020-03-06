<template>
  <v-app>
    <the-app-bar authenticated />
    <v-content>
      <v-container fluid>
        <router-view></router-view>
      </v-container>
    </v-content>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import TheAppBar from "@/components/TheAppBar.vue";
import { Component } from "vue-property-decorator";

@Component({ components: { TheAppBar } })
export default class extends Vue {}
</script>
<style>
@import "../styles/hide-scrollbar.scss";
</style>
