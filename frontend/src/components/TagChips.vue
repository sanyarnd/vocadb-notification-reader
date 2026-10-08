<script setup lang="ts">
import type { Tag } from "@/api/dto";
import { stringToColor } from "@/utils/color";

defineProps<{ tags: Tag[] }>();
const emit = defineEmits<{ select: [name: string] }>();
</script>

<template>
  <v-chip-group column>
    <v-chip
      v-for="tag in tags"
      :key="tag.id"
      variant="outlined"
      size="small"
      :color="stringToColor(tag.name)"
      @click.stop="emit('select', tag.name)"
    >
      {{ tag.name }}
      <v-avatar end class="text-caption">{{ tag.count }}</v-avatar>
    </v-chip>
  </v-chip-group>
</template>
