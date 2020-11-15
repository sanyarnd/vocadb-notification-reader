import { LoginType } from "@/plugins/store/user-module";

export interface AuthenticationPayload {
  username: string;
  password: string;
  loginType: LoginType;
}

export type VocaDbNotification = SongNotification | UnknownNotification;

export type NotificationType =
  | "AlbumNotification"
  | "ArtistNotification"
  | "EventNotification"
  | "ReportNotification"
  | "SongNotification"
  | "UnknownNotification";

export type SongNotificationType = "NEW" | "TAGGED";
export type PvType = "Original" | "Reprint" | "Other";

export const songTypeData = [
  "Unspecified",
  "Original",
  "Remaster",
  "Remix",
  "Cover",
  "Arrangement",
  "Instrumental",
  "Mashup",
  "MusicPV",
  "DramaPV",
  "Live",
  "Illustration",
  "Other"
];
export type SongType = typeof songTypeData[number];

export const pvServiceData = [
  "NicoNicoDouga",
  "Youtube",
  "SoundCloud",
  "Vimeo",
  "Piapro",
  "Bilibili",
  "File",
  "LocalFile",
  "Creofuga",
  "Bandcamp"
] as const;
export type PvService = typeof pvServiceData[number];

export interface PV {
  id: number;
  pvType: PvType;
  service: PvService;
  url: string;
  name: string;
  disabled: boolean;
  author: string | null;
  publishDate: string | null;
  pvId: string | null;
  thumbUrl: string | null;
  timestamp: string | null;
}

export interface Tag {
  count: number;
  id: number;
  name: string;
  categoryName: string | null;
}

export interface AccountData {
  vocaDbId: number;
  username: string;
}

export interface NotificationsDTO {
  totalCount: number;
  notifications: VocaDbNotification[];
}

// Notifications hierarchy

export interface BaseNotification {
  id: number;
  originalSubject: string;
  originalBody: string;
  notificationType: NotificationType;
}

export interface SongNotification extends BaseNotification {
  songId: number;
  type: SongNotificationType;
  songType: SongType;
  title: string | null;
  artist: string | null;
  tags: Tag[];
  pvs: PV[];
  releaseDate: string | null;
}

export type AlbumNotification = BaseNotification;
export type ArtistNotification = BaseNotification;
export type EventNotification = BaseNotification;
export type ReportNotification = BaseNotification;
export type UnknownNotification = BaseNotification;

export function isSongNotification(n: VocaDbNotification): n is SongNotification {
  return n.notificationType === "SongNotification";
}

export function isAlbumNotification(n: VocaDbNotification): n is AlbumNotification {
  return n.notificationType === "AlbumNotification";
}

export function isArtistNotification(n: VocaDbNotification): n is ArtistNotification {
  return n.notificationType === "ArtistNotification";
}

export function isEventNotification(n: VocaDbNotification): n is EventNotification {
  return n.notificationType === "EventNotification";
}

export function isReportNotification(n: VocaDbNotification): n is ReportNotification {
  return n.notificationType === "ReportNotification";
}

export function isUnknownNotification(n: VocaDbNotification): n is UnknownNotification {
  return n.notificationType === "UnknownNotification";
}
