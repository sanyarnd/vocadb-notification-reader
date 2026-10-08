export const databases = ["VocaDb", "TouhouDb", "UtaiteDb"] as const;
export type Database = (typeof databases)[number];

export const requestLanguages = ["Default", "Japanese", "Romaji", "English"] as const;
export type RequestLanguage = (typeof requestLanguages)[number];

export interface AuthenticationPayload {
  username: string;
  password: string;
  database: Database;
}

export interface AccessToken {
  token: string;
}

export interface MainPicture {
  urlSmallThumb: string | null;
  urlThumb: string | null;
  urlTinyThumb: string | null;
}

export interface AccountData {
  id: number;
  name: string;
  active: boolean;
  memberSince: string;
  verifiedArtist: boolean;
  groupId: string;
  mainPicture: MainPicture | null;
}

export type NotificationType =
  | "AlbumNotification"
  | "ArtistNotification"
  | "EventNotification"
  | "ReportNotification"
  | "SongNotification"
  | "UnknownNotification";

export type SongNotificationType = "New" | "Tagged";
export type PvType = "Original" | "Reprint" | "Other";

export const songTypes = [
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
] as const;
export type SongType = (typeof songTypes)[number];

export const pvServices = [
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
export type PvService = (typeof pvServices)[number];

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
  id: number;
  name: string;
  count: number;
  categoryName: string | null;
}

interface BaseNotification<T extends NotificationType> {
  notificationType: T;
  id: number;
  originalSubject: string;
  originalBody: string;
  created_date: string;
}

export interface SongNotification extends BaseNotification<"SongNotification"> {
  type: SongNotificationType;
  songId: number;
  songType: SongType;
  title: string;
  artist: string;
  tags: Tag[];
  pvs: PV[];
  releaseDate: string | null;
}

export type AlbumNotification = BaseNotification<"AlbumNotification">;
export type ArtistNotification = BaseNotification<"ArtistNotification">;
export type EventNotification = BaseNotification<"EventNotification">;
export type ReportNotification = BaseNotification<"ReportNotification">;
export type UnknownNotification = BaseNotification<"UnknownNotification">;

export type VocaDbNotification =
  | SongNotification
  | AlbumNotification
  | ArtistNotification
  | EventNotification
  | ReportNotification
  | UnknownNotification;

export interface NotificationsResponse {
  totalCount: number;
  notifications: VocaDbNotification[];
}

export function isSongNotification(n: VocaDbNotification): n is SongNotification {
  return n.notificationType === "SongNotification";
}
