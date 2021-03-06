<template>
  <div class="text-center">
    <v-card>
      <v-tabs
        v-model="selectedTab"
        centered
        grow
        @change="updateTabNotifications"
      >
        <v-tab v-for="tabType in tabs" :key="tabType">
          <v-badge
            inline
            :color="tabColor(tabType)"
            :content="tabNotificationCount(tabType)"
          >
            <v-icon>{{ tabIcon(tabType) }}</v-icon>
            {{ $vuetify.lang.t(`$vuetify.notification.type.${tabType}`) }}
          </v-badge>
        </v-tab>
      </v-tabs>

      <v-data-table
        v-model="selected"
        item-key="id"
        :headers="selectedTabHeaders"
        :items="tabNotifications"
        :search="searchQuery"
        :custom-filter="customTableFilter"
        :items-per-page="itemsPerPage"
        :loading="loading || deleteInProgress"
        height="calc(100vh - 300px)"
        fixed-header
        show-select
        hide-default-footer
        @click:row="notificationSelectHandle"
      >
        <template #top>
          <v-text-field
            v-model="searchQuery"
            :label="$vuetify.lang.t('$vuetify.notification.search')"
            :disabled="loading"
            filled
            clearable
            hide-details
          />
        </template>
        <template #[`item.type`]="{ item }">
          <v-chip color="primary">
            <v-icon>{{ iconForType(item.type) }}</v-icon>
          </v-chip>
        </template>
        <template #[`item.tags`]="{ item }">
          <v-chip-group column>
            <v-chip
              v-for="tag in item.tags"
              :key="tag.name"
              outlined
              :color="stringToColor(tag.name)"
              @click.stop="searchQuery = tag.name"
            >
              {{ tag.name }}
              <v-avatar right>{{ tag.count }}</v-avatar>
            </v-chip>
          </v-chip-group>
        </template>
        <template #[`item.releaseDate`]="{ item }">
          {{
            new Date(item.releaseDate).toLocaleDateString(locale, {
              weekday: "short",
              day: "numeric",
              month: "short",
              year: "numeric"
            })
          }}
        </template>
        <template #[`item.originalBody`]="{ item }">
          {{ removeMarkdown(item.originalBody) }}
        </template>
      </v-data-table>

      <v-snackbar
        v-model="showError"
        color="error"
        type="primary"
        transition="scale-transition"
      >
        {{ errorMessage }}
        <template #action="{ attrs }">
          <v-btn text @click="showError = false">{{
            $vuetify.lang.t("$vuetify.close")
          }}</v-btn>
        </template>
      </v-snackbar>

      <song-notification-popup
        :show-dialog="clicked != null"
        :close-dialog="() => (clicked = null)"
        :notification="clicked"
        :delete-notification="deleteNotifications"
      />
    </v-card>
    <v-btn
      width="80%"
      class="ma-3"
      :disabled="selected.length === 0 || loading || deleteInProgress"
      color="error"
      type="primary"
      @click="deleteNotifications(selected)"
    >
      {{ $vuetify.lang.t("$vuetify.delete") }}
    </v-btn>
    <v-pagination
      class="ma-3"
      :value="page"
      :length="numberOfPages()"
      :total-visible="7"
      :disabled="loading"
      @input="changePage"
    ></v-pagination>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Watch } from "vue-property-decorator";
import { DataTableHeader } from "vuetify";
import SongNotificationPopup from "@/components/SongNotificationPopup.vue";
import { api } from "@/backend";
import { stringToColor } from "@/utils";
import {
  isAlbumNotification,
  isArtistNotification,
  isEventNotification,
  isReportNotification,
  isSongNotification,
  isUnknownNotification,
  SongNotification,
  SongNotificationType,
  Tag,
  VocaDbNotification
} from "@/backend/dto";
import { Locale, settingsModule } from "@/plugins/store/settings-module";

@Component({ components: { SongNotificationPopup } })
export default class extends Vue {
  private readonly settingsStore = settingsModule.context(this.$store);

  private markdownUrlDescriptionPattern = new RegExp("\\[(.*)\\]\\((.*?)\\)");

  private notifications: VocaDbNotification[] = [];
  private tabNotifications: VocaDbNotification[] = [];
  private selected: VocaDbNotification[] = [];
  private clicked: SongNotification | null = null;

  private searchQuery = "";
  private page = 1;
  private totalNotificationsCount = 1;

  private loading = false;
  private deleteInProgress = false;
  private showError = false;
  private errorMessage = "";

  private tabs = tabTypes;
  private selectedTab = 0;

  mounted(): void {
    this.fetchNotifications(1);
  }

  get locale(): Locale {
    return this.settingsStore.getters.locale;
  }

  get itemsPerPage(): number {
    return this.settingsStore.getters.itemsPerPage;
  }

  @Watch("settingsStore.getters.preferredLanguage")
  onLocaleChange(): void {
    this.fetchNotifications(this.page);
  }

  @Watch("settingsStore.getters.itemsPerPage")
  onItemsPerPageChange(): void {
    this.changePage(1);
  }

  private iconForType(type: SongNotificationType): string {
    switch (type) {
      case "New":
        return "mdi-music";
      case "Tagged":
        return "mdi-tag-text";
    }
  }

  private tabColor(tab: TabType): string {
    if (this.tabNotificationCount(tab) === "0") {
      return this.settingsStore.getters.isDarkTheme
        ? "grey darken-3"
        : "grey lighten";
    } else {
      return "primary";
    }
  }

  private tabNotificationCount(type: TabType): string {
    return tabsHelper[type].count(this.notifications).toString();
  }

  private get selectedTabHeaders(): DataTableHeader[] {
    return tabsHelper[this.selectedTabType].headers;
  }

  private get selectedTabType(): TabType {
    return tabTypes[this.selectedTab];
  }

  private tabIcon(type: TabType): string {
    return tabsHelper[type].icon;
  }

  private removeMarkdown(markdown: string): string {
    return markdown.replace(this.markdownUrlDescriptionPattern, "$1");
  }

  private extractUrlFromMarkdown(body: string): string | null {
    const regex = this.markdownUrlDescriptionPattern.exec(body);
    if (regex === null || regex.length != 3) return null;
    return regex[2];
  }

  customTableFilter(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/explicit-module-boundary-types
    value: any,
    search: string | null,
    item: VocaDbNotification // eslint-disable-line @typescript-eslint/no-unused-vars
  ): boolean {
    if (value === null || search === null || typeof value === "boolean") {
      return false;
    }

    const searchLowerCase = search.toLocaleLowerCase();
    if (Array.isArray(value)) {
      // there is only one array in the table
      const tags = value as Tag[];
      return (
        tags.filter(t => {
          const loweCaseTag = t.name.toLocaleLowerCase();
          return loweCaseTag.includes(searchLowerCase);
        }).length > 0
      );
    } else {
      return value.toString().toLocaleLowerCase().includes(searchLowerCase);
    }
  }

  updateTabNotifications(): void {
    this.tabNotifications = tabsHelper[this.selectedTabType].notifications(
      this.notifications
    );
  }

  changePage(page: number): void {
    const previousPage = this.page;
    this.page = page;
    this.selected = [];
    this.fetchNotifications(previousPage);
  }

  numberOfPages(): number {
    return Math.ceil(this.totalNotificationsCount / this.itemsPerPage);
  }

  notificationSelectHandle(value: VocaDbNotification): void {
    if (isSongNotification(value)) {
      if (this.loading || this.deleteInProgress || this.clicked !== null)
        return;
      this.clicked = value;
    } else {
      const url = this.extractUrlFromMarkdown(value.originalBody);
      if (url === null) {
        console.log("Unable to extract URL, no window popup");
        return;
      }
      window.open(url, "_blank");
    }
  }

  async fetchNotifications(previousPage: number): Promise<void> {
    this.loading = true;
    try {
      const preferredLanguage = this.settingsStore.getters.preferredLanguage;
      const response = await api.getNotifications(
        this.itemsPerPage,
        this.itemsPerPage * (this.page - 1),
        preferredLanguage
      );
      this.notifications = response.data.notifications;
      this.totalNotificationsCount = response.data.totalCount;

      this.updateTabNotifications();
    } catch (e) {
      this.showErrorMessage();
      this.page = previousPage;
    } finally {
      this.loading = false;
    }
  }

  private async deleteNotifications(notifications: VocaDbNotification[]) {
    this.clicked = null;
    this.deleteInProgress = true;

    const toDeleteIds: number[] = notifications.map(ntf => ntf.id);
    try {
      await api.deleteNotifications(toDeleteIds);
      this.notifications = this.notifications.filter(
        ntf => !toDeleteIds.includes(ntf.id)
      );
      this.selected = this.selected.filter(
        ntf => !toDeleteIds.includes(ntf.id)
      );
      this.updateTabNotifications();
    } catch (e) {
      this.showErrorMessage();
    } finally {
      this.deleteInProgress = false;
    }
  }

  private showErrorMessage() {
    this.showError = true;
    this.errorMessage = this.$vuetify.lang.t("$vuetify.connectionError");
  }

  private stringToColor(str: string): string {
    return stringToColor(str);
  }
}

const tabTypes = [
  "song",
  "artist",
  "event",
  "album",
  "report",
  "unknown"
] as const;
type TabType = typeof tabTypes[number];

interface TabData {
  headers: DataTableHeader[];
  icon: string;
  count: (notifications: VocaDbNotification[]) => number;
  notifications: (notifications: VocaDbNotification[]) => VocaDbNotification[];
}

const tabsHelper: Record<TabType, TabData> = {
  song: {
    headers: [
      { value: "type", text: "Type", width: "5%" },
      { value: "songType", text: "Song Type", width: "5%" },
      { value: "title", text: "Title", width: "20%" },
      { value: "artist", text: "Artist", width: "20%" },
      { value: "tags", text: "Tags", width: "35%", sortable: false },
      { value: "releaseDate", text: "Release Date", width: "15%" }
    ],
    icon: "mdi-music-note",
    count: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isSongNotification(n)).length,
    notifications: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isSongNotification(n))
  },
  album: {
    headers: [
      { value: "originalSubject", text: "Subject", width: "30%" },
      { value: "originalBody", text: "Text", width: "70%" }
    ],
    icon: "mdi-album",
    count: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isAlbumNotification(n)).length,
    notifications: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isAlbumNotification(n))
  },
  artist: {
    headers: [
      { value: "originalSubject", text: "Subject", width: "30%" },
      { value: "originalBody", text: "Text", width: "70%" }
    ],
    icon: "mdi-account-music",
    count: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isArtistNotification(n)).length,
    notifications: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isArtistNotification(n))
  },
  report: {
    headers: [
      { value: "originalSubject", text: "Subject", width: "30%" },
      { value: "originalBody", text: "Text", width: "70%" }
    ],
    icon: "mdi-alert",
    count: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isReportNotification(n)).length,
    notifications: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isReportNotification(n))
  },
  event: {
    headers: [
      { value: "originalSubject", text: "Subject", width: "30%" },
      { value: "originalBody", text: "Text", width: "70%" }
    ],
    icon: "mdi-calendar",
    count: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isEventNotification(n)).length,
    notifications: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isEventNotification(n))
  },
  unknown: {
    headers: [
      { value: "originalSubject", text: "Subject", width: "30%" },
      { value: "originalBody", text: "Text", width: "70%" }
    ],
    icon: "mdi-help",
    count: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isUnknownNotification(n)).length,
    notifications: (notifications: VocaDbNotification[]) =>
      notifications.filter(n => isUnknownNotification(n))
  }
};
</script>

<style lang="scss" scoped>
.v-chip--outlined.v-chip--active::before {
  opacity: 0;
}
</style>
