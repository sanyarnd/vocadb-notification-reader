<script setup lang="ts" generic="T extends string | number">
export interface SettingOption<V> {
  value: V;
  title: string;
}

const props = defineProps<{
  icon: string;
  label: string;
  items: SettingOption<T>[];
}>();
const model = defineModel<T>({ required: true });
</script>

<template>
  <v-menu>
    <template #activator="{ props: activator }">
      <v-btn v-bind="activator" variant="text" size="small" :prepend-icon="props.icon">
        {{ label }}
        <v-icon end>mdi-menu-down</v-icon>
      </v-btn>
    </template>
    <v-list density="compact">
      <v-list-item
        v-for="item in props.items"
        :key="item.value"
        :active="item.value === model"
        :title="item.title"
        @click="model = item.value"
      />
    </v-list>
  </v-menu>
</template>
