import type { NotificationType, VocaDbNotification } from "@/api/dto";
import { isSongNotification } from "@/api/dto";

export const tabTypes = ["song", "artist", "event", "album", "report", "unknown"] as const;
export type TabType = (typeof tabTypes)[number];

interface TabData {
  notificationType: NotificationType;
  icon: string;
}

export const tabs: Record<TabType, TabData> = {
  song: { notificationType: "SongNotification", icon: "mdi-music-note" },
  artist: { notificationType: "ArtistNotification", icon: "mdi-account-music" },
  event: { notificationType: "EventNotification", icon: "mdi-calendar" },
  album: { notificationType: "AlbumNotification", icon: "mdi-album" },
  report: { notificationType: "ReportNotification", icon: "mdi-alert" },
  unknown: { notificationType: "UnknownNotification", icon: "mdi-help" }
};

export function notificationsForTab(
  notifications: VocaDbNotification[],
  tab: TabType
): VocaDbNotification[] {
  return notifications.filter(n => n.notificationType === tabs[tab].notificationType);
}

/** Case-insensitive search over the visible notification fields and song tags. */
export function matchesSearch(notification: VocaDbNotification, query: string | null): boolean {
  const needle = query?.trim().toLocaleLowerCase() ?? "";
  if (needle === "") return true;

  const haystack: (string | null)[] = isSongNotification(notification)
    ? [
        notification.title,
        notification.artist,
        notification.songType,
        notification.type,
        notification.releaseDate,
        ...notification.tags.map(t => t.name)
      ]
    : [notification.originalSubject, notification.originalBody];

  return haystack.some(value => value?.toLocaleLowerCase().includes(needle));
}
