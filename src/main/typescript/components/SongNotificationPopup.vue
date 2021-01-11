<template>
  <v-dialog
    v-model="showDialog"
    transition="dialog-bottom-transition"
    fullscreen
    hide-overlay
    @keydown.esc="closeDialog"
  >
    <v-card>
      <v-toolbar elevation="0">
        <v-btn icon @click="closeDialog">
          <v-icon>mdi-close</v-icon>
        </v-btn>
        <v-toolbar-title>{{ notificationTitle }}</v-toolbar-title>
        <v-spacer />
        <v-btn
          class="mr-3"
          :href="'https://vocadb.net/S/' + songUrl"
          target="_blank"
        >
          <v-icon left>mdi-arrow-right</v-icon>
          To VocaDB
        </v-btn>
        <v-btn type="primary" @click="deleteNotification([notification])">
          {{ $vuetify.lang.t("delete") }}
          <v-icon right>mdi-close</v-icon>
        </v-btn>
      </v-toolbar>

      <v-card elevation="0">
        Title: {{ songTitle }} <br />
        Artist: {{ songArtist }} <br />
        Release date: {{ songReleaseDate }}
      </v-card>

      <v-chip-group>
        <v-chip
          v-for="tag in tags"
          :key="tag.name"
          outlined
          :color="stringToColor(tag.name)"
        >
          {{ tag.name }}
          <v-avatar right>{{ tag.count }}</v-avatar>
        </v-chip>
      </v-chip-group>

      <v-tabs v-model="selectedPv" grow>
        <v-tab v-for="pv in notificationPvs" :key="pv.id">
          <v-avatar tile size="24"
            ><img :src="iconForService(pv.service)"
          /></v-avatar>
          {{ pv.service }}
        </v-tab>

        <v-tab-item v-for="pv in notificationPvs" :key="pv.id">
          <v-card width="100vw" height="calc(100vh - 240px)">
            <iframe
              style="width: 100%; height: calc(100vh - 240px)"
              :src="embedForPv(pv.service, pv.pvId, pv.timestamp)"
              frameborder="0"
              allowfullscreen
            />
          </v-card>
        </v-tab-item>
      </v-tabs>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop, Watch } from "vue-property-decorator";
import { Locale, settingsModule } from "@/plugins/store/settings-module";
import {
  PV,
  PvService,
  SongNotification,
  Tag,
  VocaDbNotification
} from "@/backend/dto";
import { stringToColor } from "@/utils";

@Component
export default class extends Vue {
  @Prop({ default: false })
  private readonly showDialog!: boolean;

  @Prop()
  private readonly notification!: SongNotification | null;

  @Prop()
  private readonly closeDialog!: () => void;

  @Prop()
  private readonly deleteNotification!: (
    notifications: VocaDbNotification[]
  ) => void;

  private readonly settingsStore = settingsModule.context(this.$store);

  private selectedPv = 0;

  @Watch("showDialog")
  selectPreferredService(value: boolean): void {
    if (value) {
      const id = this.notificationPvs
        .map(pv => pv.service)
        .indexOf(this.settingsStore.getters.preferredPvService);
      if (id !== -1) this.selectedPv = id;
    }
  }

  get locale(): Locale {
    return this.settingsStore.getters.locale;
  }

  get notificationTitle(): string {
    if (this.notification === null) return "";
    return this.notification.originalSubject;
  }

  get notificationPvs(): PV[] {
    if (this.notification === null) return [];
    return this.notification.pvs.filter(pv => !pv.disabled);
  }

  get songArtist(): string {
    if (this.notification === null || this.notification.artist === null)
      return "<no-data>";
    return this.notification.artist;
  }

  get songTitle(): string {
    if (this.notification === null || this.notification.title === null)
      return "<no-data>";
    return this.notification.title;
  }

  get songReleaseDate(): string {
    if (this.notification === null || this.notification.releaseDate === null)
      return "<no-data>";
    return new Date(this.notification.releaseDate).toLocaleDateString(
      this.locale,
      {
        weekday: "short",
        day: "numeric",
        month: "short",
        year: "numeric"
      }
    );
  }

  get songUrl(): string {
    if (this.notification === null || this.notification.songId === null)
      return "<no-data>";
    return this.notification.songId.toString();
  }

  get tags(): Tag[] | null {
    if (this.notification === null) return [];
    return (this.notification as SongNotification).tags;
  }

  iconForService(pvService: PvService): string {
    switch (pvService) {
      case "NicoNicoDouga":
        return require(`@/assets/services/nico.webp`);
      case "Youtube":
        return require(`@/assets/services/youtube.svg`);
      case "SoundCloud":
        return require(`@/assets/services/soundcloud.svg`);
      case "Vimeo":
        return require(`@/assets/services/vimeo.svg`);
      case "Piapro":
        return require(`@/assets/services/piapro.png`);
      case "Bilibili":
        return require(`@/assets/services/bilibili.svg`);
      case "File":
        return require(`@/assets/services/file.svg`);
      case "LocalFile":
        return require(`@/assets/services/file.svg`);
      case "Creofuga":
        return require(`@/assets/services/creofuga.png`);
      case "Bandcamp":
        return require(`@/assets/services/bandcamp.svg`);
    }
  }

  embedForPv(service: PvService, id: string, timestamp: string | null): string {
    switch (service) {
      case "NicoNicoDouga":
        return `https://embed.nicovideo.jp/watch/${id}?jsapi=0&noRelatedVideo=0&autoplay=0&defaultNoComment=0&noLinkToNiconico=0&noController=0&noHeader=0&noTags=0&noShare=0`;
      case "Youtube":
        return `https://www.youtube.com/embed/${id}`;
      case "SoundCloud":
        id = id.split(" ")[0].trim();
        return `https://w.soundcloud.com/player/?url=https%3A%2F%2Fapi.soundcloud.com%2Ftracks%2F${id}`;
      case "Vimeo":
        return `https://player.vimeo.com/video/${id}`;
      case "Piapro":
        if (timestamp == null) {
          return `https://piapro.jp/html5_player_popup/?id=${id}`;
        }
        return `https://piapro.jp/html5_player_popup/?id=${id}&cdate=${timestamp}`;
      case "Bilibili":
        return `https://player.bilibili.com/player.html?aid=${id}`;
      case "File":
        return `${id}`;
      case "LocalFile":
        return `${id}`;
      case "Creofuga":
        return `https://creofuga.net/audios/player?id=${id}`;
      case "Bandcamp":
        return `https://bandcamp.com/EmbeddedPlayer/size=large/bgcol=ffffff/linkcol=0687f5/tracklist=false/artwork=small/track=${id}/transparent=true/`;
    }
  }

  private stringToColor(str: string): string {
    return stringToColor(str);
  }
}
</script>

<style lang="scss" scoped>
.v-chip--outlined.v-chip--active::before {
  opacity: 0;
}
</style>
