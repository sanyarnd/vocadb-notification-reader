import { describe, expect, it } from "vitest";

import { stringToColor } from "@/utils/color";
import { songUrl } from "@/utils/database";
import { formatDate } from "@/utils/date";
import { extractUrlFromMarkdown, removeMarkdown } from "@/utils/markdown";
import { matchesSearch, notificationsForTab } from "@/utils/notifications";
import { embedUrl, iconForService } from "@/utils/pv";

import { artistNotification, pv, songNotification } from "./fixtures";

describe("stringToColor", () => {
  it("is deterministic and produces hsl colors", () => {
    expect(stringToColor("rock")).toBe(stringToColor("rock"));
    expect(stringToColor("rock")).not.toBe(stringToColor("pop"));
    expect(stringToColor("rock")).toMatch(/^hsl\(\d+, \d+%, \d+%\)$/);
  });

  it("handles an empty string", () => {
    expect(stringToColor("")).toBe("hsl(0, 0%, 50%)");
  });
});

describe("markdown", () => {
  const body = "New song: [Melt](https://vocadb.net/S/100) by ryo";

  it("replaces a link with its description", () => {
    expect(removeMarkdown(body)).toBe("New song: Melt by ryo");
    expect(removeMarkdown("plain text")).toBe("plain text");
  });

  it("extracts a link URL", () => {
    expect(extractUrlFromMarkdown(body)).toBe("https://vocadb.net/S/100");
    expect(extractUrlFromMarkdown("plain text")).toBeNull();
  });
});

describe("songUrl", () => {
  it("points to the database the user logged in to", () => {
    expect(songUrl("VocaDb", 1)).toBe("https://vocadb.net/S/1");
    expect(songUrl("TouhouDb", 2)).toBe("https://touhoudb.com/S/2");
    expect(songUrl("UtaiteDb", 3)).toBe("https://utaitedb.net/S/3");
  });
});

describe("formatDate", () => {
  it("formats valid dates and rejects the rest", () => {
    expect(formatDate("2007-12-07T00:00:00Z", "en")).toContain("2007");
    expect(formatDate(null, "en")).toBeNull();
    expect(formatDate("not a date", "en")).toBeNull();
  });
});

describe("pv", () => {
  it("builds embed URLs", () => {
    expect(embedUrl(pv({ service: "Youtube", pvId: "abc" }))).toBe(
      "https://www.youtube.com/embed/abc"
    );
    expect(embedUrl(pv({ service: "NicoNicoDouga", pvId: "sm9" }))).toMatch(
      /^https:\/\/embed\.nicovideo\.jp\/watch\/sm9\?/
    );
    expect(embedUrl(pv({ service: "SoundCloud", pvId: "123 user/track" }))).toBe(
      "https://w.soundcloud.com/player/?url=https%3A%2F%2Fapi.soundcloud.com%2Ftracks%2F123"
    );
    expect(embedUrl(pv({ service: "Piapro", pvId: "p1", timestamp: null }))).toBe(
      "https://piapro.jp/html5_player_popup/?id=p1"
    );
    expect(embedUrl(pv({ service: "Piapro", pvId: "p1", timestamp: "2007" }))).toBe(
      "https://piapro.jp/html5_player_popup/?id=p1&cdate=2007"
    );
    expect(embedUrl(pv({ service: "File", pvId: null, url: "https://x/y.mp3" }))).toBe(
      "https://x/y.mp3"
    );
  });

  it("has an icon for every service", () => {
    expect(iconForService("Bandcamp")).toBeTruthy();
    expect(iconForService("LocalFile")).toBe(iconForService("File"));
  });
});

describe("notifications", () => {
  const song = songNotification();
  const artist = artistNotification(2, "[Miku](https://vocadb.net/Ar/1)");

  it("splits notifications by tab", () => {
    expect(notificationsForTab([song, artist], "song")).toEqual([song]);
    expect(notificationsForTab([song, artist], "artist")).toEqual([artist]);
    expect(notificationsForTab([song, artist], "album")).toEqual([]);
  });

  it("searches song fields and tags case-insensitively", () => {
    expect(matchesSearch(song, null)).toBe(true);
    expect(matchesSearch(song, "  ")).toBe(true);
    expect(matchesSearch(song, "MELT")).toBe(true);
    expect(matchesSearch(song, "miku")).toBe(true);
    expect(matchesSearch(song, "roc")).toBe(true);
    expect(matchesSearch(song, "jazz")).toBe(false);
  });

  it("searches the subject and body of other notifications", () => {
    expect(matchesSearch(artist, "new art")).toBe(true);
    expect(matchesSearch(artist, "miku")).toBe(true);
    expect(matchesSearch(artist, "melt")).toBe(false);
  });
});
