<template>
  <v-main>
    <the-app-bar />
    <v-container fill-height>
      <v-layout align-center justify-center>
        <login-dialog />
      </v-layout>
    </v-container>
  </v-main>
</template>

<script lang="ts">
import Vue from "vue";
import { Component } from "vue-property-decorator";
import LoginDialog from "@/components/LoginDialog.vue";
import TheAppBar from "@/components/TheAppBar.vue";

@Component({ components: { TheAppBar, LoginDialog } })
export default class extends Vue {}
</script>
