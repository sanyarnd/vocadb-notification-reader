import { en as vuetifyEn, ja as vuetifyJa, ru as vuetifyRu } from "vuetify/locale";

import { en } from "./en";
import { ja } from "./ja";
import { ru } from "./ru";

export type { Messages } from "./en";

export const messages = {
  en: { ...vuetifyEn, ...en },
  ja: { ...vuetifyJa, ...ja },
  ru: { ...vuetifyRu, ...ru }
};
