import type { Account, PV, SongNotification, VocaDbNotification } from "@/api/dto";

export function pv(overrides: Partial<PV> = {}): PV {
  return {
    id: 1,
    pvType: "Original",
    service: "NicoNicoDouga",
    url: "https://www.nicovideo.jp/watch/sm1",
    name: "PV",
    disabled: false,
    author: null,
    publishDate: null,
    pvId: "sm1",
    thumbUrl: null,
    timestamp: null,
    ...overrides
  };
}

export function songNotification(overrides: Partial<SongNotification> = {}): SongNotification {
  return {
    notificationType: "SongNotification",
    id: 1,
    originalSubject: "New song tagged with rock",
    originalBody: "[Melt](https://vocadb.net/S/100)",
    createdDate: "2022-02-25T14:29:00Z",
    type: "Tagged",
    songId: 100,
    songType: "Original",
    title: "Melt",
    artist: "ryo feat. Hatsune Miku",
    tags: [{ id: 1, name: "rock", count: 3, categoryName: "Genres" }],
    pvs: [pv()],
    releaseDate: "2007-12-07T00:00:00Z",
    ...overrides
  };
}

export function artistNotification(id: number, body: string): VocaDbNotification {
  return {
    notificationType: "ArtistNotification",
    id,
    originalSubject: "New artist",
    originalBody: body,
    createdDate: "2022-02-25T14:29:00Z"
  };
}

export function account(overrides: Partial<Account> = {}): Account {
  return {
    database: "VocaDb",
    user: {
      id: 1,
      name: "miku",
      active: true,
      memberSince: "2020-01-01",
      verifiedArtist: false,
      groupId: "Regular",
      mainPicture: null
    },
    ...overrides
  };
}
