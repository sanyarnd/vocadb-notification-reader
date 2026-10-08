// API types are generated from the backend (`cargo test` in `backend/`), see `./generated`.
import type { Database } from "./generated/Database";
import type { LanguagePreference } from "./generated/LanguagePreference";
import type { Notification } from "./generated/Notification";
import type { PvService } from "./generated/PvService";
import type { SongType } from "./generated/SongType";

export type { Account } from "./generated/Account";
export type { Database } from "./generated/Database";
export type { ErrorResponse } from "./generated/ErrorResponse";
export type { LoginRequest } from "./generated/LoginRequest";
export type { NotificationsResponse } from "./generated/NotificationsResponse";
export type { PV } from "./generated/PV";
export type { PvService } from "./generated/PvService";
export type { PvType } from "./generated/PvType";
export type { SongNotificationType } from "./generated/SongNotificationType";
export type { SongType } from "./generated/SongType";
export type { Tag } from "./generated/Tag";
export type { UserForApiContract } from "./generated/UserForApiContract";

export type RequestLanguage = LanguagePreference;
export type VocaDbNotification = Notification;
export type NotificationType = Notification["notificationType"];
export type SongNotification = Extract<Notification, { notificationType: "SongNotification" }>;

/** Lists every member of a string union; fails to compile when one is missing. */
function allOf<T extends string>() {
  return <const A extends readonly T[]>(
    values: A & ([Exclude<T, A[number]>] extends [never] ? unknown : never)
  ): A => values;
}

export const databases = allOf<Database>()(["VocaDb", "TouhouDb", "UtaiteDb"]);
export const requestLanguages = allOf<RequestLanguage>()([
  "Default",
  "Japanese",
  "Romaji",
  "English"
]);
export const songTypes = allOf<SongType>()([
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
]);
export const pvServices = allOf<PvService>()([
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
]);

export function isSongNotification(n: VocaDbNotification): n is SongNotification {
  return n.notificationType === "SongNotification";
}
