<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useLocale } from "vuetify";

import type { SongNotification } from "@/api/dto";
import TagChips from "@/components/TagChips.vue";
import { useAccountStore } from "@/stores/account";
import { useSettingsStore } from "@/stores/settings";
import { songUrl } from "@/utils/database";
import { formatDate } from "@/utils/date";
import { embedUrl, iconForService } from "@/utils/pv";

const props = defineProps<{ notification: SongNotification | null }>();
const emit = defineEmits<{ close: []; delete: [id: number] }>();

const account = useAccountStore();
const settings = useSettingsStore();
const { t } = useLocale();

const selectedPv = ref<number | null>(null);

const open = computed({
  get: () => props.notification !== null,
  set: value => {
    if (!value) emit("close");
  }
});

const pvs = computed(() => props.notification?.pvs.filter(pv => !pv.disabled) ?? []);
const releaseDate = computed(() =>
  formatDate(props.notification?.releaseDate ?? null, settings.locale)
);
const databaseUrl = computed(() =>
  props.notification === null ? undefined : songUrl(account.database, props.notification.songId)
);

watch(
  () => props.notification,
  notification => {
    if (notification === null) return;
    const preferred = pvs.value.find(pv => pv.service === settings.preferredPvService);
    selectedPv.value = (preferred ?? pvs.value[0])?.id ?? null;
  }
);
</script>

<template>
  <v-dialog v-model="open" transition="dialog-bottom-transition" fullscreen>
    <v-card v-if="notification">
      <v-toolbar>
        <v-btn icon="mdi-close" @click="emit('close')" />
        <v-toolbar-title>{{ notification.originalSubject }}</v-toolbar-title>
        <v-btn
          class="mr-3"
          variant="text"
          prepend-icon="mdi-arrow-right"
          :href="databaseUrl"
          target="_blank"
          rel="noopener"
        >
          {{ t("$vuetify.toDatabase", t(`$vuetify.database.${account.database}`)) }}
        </v-btn>
        <v-btn
          color="error"
          variant="flat"
          append-icon="mdi-delete"
          @click="emit('delete', notification.id)"
        >
          {{ t("$vuetify.delete") }}
        </v-btn>
      </v-toolbar>

      <v-card-text>
        <div>{{ t("$vuetify.notification.title") }}: {{ notification.title }}</div>
        <div>{{ t("$vuetify.notification.artist") }}: {{ notification.artist }}</div>
        <div>
          {{ t("$vuetify.notification.releaseDate") }}: {{ releaseDate ?? t("$vuetify.noData") }}
        </div>
        <tag-chips :tags="notification.tags" />
      </v-card-text>

      <v-tabs v-model="selectedPv" grow>
        <v-tab v-for="pv in pvs" :key="pv.id" :value="pv.id">
          <v-avatar rounded="0" size="24" class="mr-2">
            <v-img :src="iconForService(pv.service)" :alt="pv.service" />
          </v-avatar>
          {{ pv.service }}
        </v-tab>
      </v-tabs>
      <v-tabs-window v-model="selectedPv">
        <v-tabs-window-item v-for="pv in pvs" :key="pv.id" :value="pv.id">
          <iframe
            class="player"
            :src="embedUrl(pv)"
            :title="pv.name"
            allow="autoplay; encrypted-media; fullscreen"
          />
        </v-tabs-window-item>
      </v-tabs-window>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.player {
  width: 100%;
  height: calc(100vh - 280px);
  border: 0;
}
</style>
