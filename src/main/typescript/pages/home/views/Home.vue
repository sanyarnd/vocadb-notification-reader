<template>
  <notification-table />
</template>

<script lang="ts">
import Vue from "vue";
import { Component } from "vue-property-decorator";
import NotificationTable from "@/components/NotificationTable.vue";

@Component({ components: { NotificationTable } })
export default class extends Vue {}
</script>
