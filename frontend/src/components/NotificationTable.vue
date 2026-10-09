<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useLocale } from "vuetify";

import { isSongNotification, type SongNotification, type VocaDbNotification } from "@/api/dto";
import SongNotificationPopup from "@/components/SongNotificationPopup.vue";
import TagChips from "@/components/TagChips.vue";
import { useNotifications } from "@/composables/useNotifications";
import { useSettingsStore } from "@/stores/settings";
import { formatDate } from "@/utils/date";
import { extractUrlFromMarkdown, removeMarkdown } from "@/utils/markdown";
import { tabIcons, tabTypes } from "@/utils/notifications";

const settings = useSettingsStore();
const { t } = useLocale();
const {
  kind: selectedTab,
  search,
  notifications,
  counts,
  page,
  pageCount,
  loading,
  deleting,
  error,
  load,
  remove
} = useNotifications();

const selected = ref<number[]>([]);
const clicked = ref<SongNotification | null>(null);

const busy = computed(() => loading.value || deleting.value);
const showError = computed({
  get: () => error.value !== null,
  set: value => {
    if (!value) error.value = null;
  }
});

const headers = computed(() =>
  selectedTab.value === "song"
    ? [
        { key: "type", title: t("$vuetify.notification.header.type"), width: "5%" },
        { key: "songType", title: t("$vuetify.notification.header.songType"), width: "5%" },
        { key: "title", title: t("$vuetify.notification.header.title"), width: "20%" },
        { key: "artist", title: t("$vuetify.notification.header.artist"), width: "20%" },
        {
          key: "tags",
          title: t("$vuetify.notification.header.tags"),
          width: "35%",
          sortable: false
        },
        {
          key: "releaseDate",
          title: t("$vuetify.notification.header.releaseDate"),
          width: "15%"
        }
      ]
    : [
        {
          key: "originalSubject",
          title: t("$vuetify.notification.header.subject"),
          width: "30%"
        },
        { key: "originalBody", title: t("$vuetify.notification.header.text"), width: "70%" }
      ]
);

function changePage(target: number): void {
  selected.value = [];
  void load(target);
}

function onRowClick(_event: unknown, row: { item: VocaDbNotification }): void {
  const notification = row.item;
  if (isSongNotification(notification)) {
    if (!busy.value) clicked.value = notification;
    return;
  }
  const url = extractUrlFromMarkdown(notification.originalBody);
  if (url !== null) window.open(url, "_blank", "noopener");
}

async function deleteNotifications(ids: number[]): Promise<void> {
  clicked.value = null;
  if (await remove(ids)) {
    selected.value = selected.value.filter(id => !ids.includes(id));
  }
}

watch(selectedTab, () => (selected.value = []));
onMounted(() => load(1));
</script>

<template>
  <div class="text-center">
    <v-card>
      <v-tabs v-model="selectedTab" grow>
        <v-tab v-for="tab in tabTypes" :key="tab" :value="tab">
          <v-icon start>{{ tabIcons[tab] }}</v-icon>
          {{ t(`$vuetify.notification.type.${tab}`) }}
          <v-badge
            class="ms-2"
            inline
            :color="counts[tab] === 0 ? 'grey' : 'primary'"
            :content="counts[tab]"
          />
        </v-tab>
      </v-tabs>

      <v-text-field
        v-model="search"
        :label="t('$vuetify.notification.search')"
        :disabled="loading"
        prepend-inner-icon="mdi-magnify"
        variant="filled"
        clearable
        hide-details
      />

      <v-data-table
        v-model="selected"
        item-value="id"
        :headers="headers"
        :items="notifications"
        :items-per-page="-1"
        :loading="busy"
        height="calc(100vh - 340px)"
        fixed-header
        show-select
        hover
        hide-default-footer
        @click:row="onRowClick"
      >
        <template #[`item.type`]="{ item }">
          <v-chip v-if="isSongNotification(item)" color="primary">
            <v-icon>{{ item.type === "Tagged" ? "mdi-tag-text" : "mdi-music" }}</v-icon>
          </v-chip>
        </template>
        <template #[`item.tags`]="{ item }">
          <tag-chips
            v-if="isSongNotification(item)"
            :tags="item.tags"
            @select="name => (search = name)"
          />
        </template>
        <template #[`item.releaseDate`]="{ item }">
          <template v-if="isSongNotification(item)">
            {{ formatDate(item.releaseDate, settings.locale) }}
          </template>
        </template>
        <template #[`item.originalBody`]="{ item }">
          {{ removeMarkdown(item.originalBody) }}
        </template>
      </v-data-table>
    </v-card>

    <v-btn
      class="ma-3"
      width="80%"
      color="error"
      :disabled="selected.length === 0 || busy"
      @click="deleteNotifications(selected)"
    >
      {{ t("$vuetify.delete") }}
    </v-btn>
    <v-pagination
      class="ma-3"
      :model-value="page"
      :length="pageCount"
      :total-visible="7"
      :disabled="loading"
      @update:model-value="changePage"
    />

    <v-snackbar v-model="showError" color="error">
      {{ error ? t(error) : "" }}
      <template #actions>
        <v-btn variant="text" @click="showError = false">{{ t("$vuetify.close") }}</v-btn>
      </template>
    </v-snackbar>

    <song-notification-popup
      :notification="clicked"
      @close="clicked = null"
      @delete="id => deleteNotifications([id])"
    />
  </div>
</template>
