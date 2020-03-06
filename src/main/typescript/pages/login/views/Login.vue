<template>
  <login-dialog />
</template>

<script lang="ts">
import Vue from "vue";
import { Component } from "vue-property-decorator";
import LoginDialog from "@/components/LoginDialog.vue";

@Component({ components: { LoginDialog } })
export default class extends Vue {}
</script>
