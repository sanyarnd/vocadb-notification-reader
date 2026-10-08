import type { PV, PvService } from "@/api/dto";
import bandcamp from "@/assets/services/bandcamp.svg";
import bilibili from "@/assets/services/bilibili.svg";
import creofuga from "@/assets/services/creofuga.png";
import file from "@/assets/services/file.svg";
import nico from "@/assets/services/nico.webp";
import piapro from "@/assets/services/piapro.png";
import soundcloud from "@/assets/services/soundcloud.svg";
import vimeo from "@/assets/services/vimeo.svg";
import youtube from "@/assets/services/youtube.svg";

const serviceIcons: Record<PvService, string> = {
  NicoNicoDouga: nico,
  Youtube: youtube,
  SoundCloud: soundcloud,
  Vimeo: vimeo,
  Piapro: piapro,
  Bilibili: bilibili,
  File: file,
  LocalFile: file,
  Creofuga: creofuga,
  Bandcamp: bandcamp
};

export function iconForService(service: PvService): string {
  return serviceIcons[service];
}

/** Builds an embeddable player URL for a PV. */
export function embedUrl(pv: Pick<PV, "service" | "pvId" | "timestamp" | "url">): string {
  const id = pv.pvId ?? "";
  switch (pv.service) {
    case "NicoNicoDouga":
      return `https://embed.nicovideo.jp/watch/${id}?jsapi=0&noRelatedVideo=0&autoplay=0&defaultNoComment=0&noLinkToNiconico=0&noController=0&noHeader=0&noTags=0&noShare=0`;
    case "Youtube":
      return `https://www.youtube.com/embed/${id}`;
    case "SoundCloud":
      return `https://w.soundcloud.com/player/?url=https%3A%2F%2Fapi.soundcloud.com%2Ftracks%2F${id.split(" ")[0]!.trim()}`;
    case "Vimeo":
      return `https://player.vimeo.com/video/${id}`;
    case "Piapro":
      return pv.timestamp === null
        ? `https://piapro.jp/html5_player_popup/?id=${id}`
        : `https://piapro.jp/html5_player_popup/?id=${id}&cdate=${pv.timestamp}`;
    case "Bilibili":
      return `https://player.bilibili.com/player.html?aid=${id}`;
    case "File":
    case "LocalFile":
      return pv.pvId ?? pv.url;
    case "Creofuga":
      return `https://creofuga.net/audios/player?id=${id}`;
    case "Bandcamp":
      return `https://bandcamp.com/EmbeddedPlayer/size=large/bgcol=ffffff/linkcol=0687f5/tracklist=false/artwork=small/track=${id}/transparent=true/`;
  }
}
