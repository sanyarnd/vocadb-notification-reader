import type { NotificationKind } from "@/api/dto";

/** Tabs in display order. */
export const tabTypes = [
  "song",
  "artist",
  "event",
  "album",
  "report",
  "unknown"
] as const satisfies readonly NotificationKind[];
export type TabType = NotificationKind;

export const tabIcons: Record<NotificationKind, string> = {
  song: "mdi-music-note",
  artist: "mdi-account-music",
  event: "mdi-calendar",
  album: "mdi-album",
  report: "mdi-alert",
  unknown: "mdi-help"
};
