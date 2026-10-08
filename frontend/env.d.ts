/// <reference types="vite/client" />

interface ImportMetaEnv {
  /** Backend base URL; requests go to the same origin when empty. */
  readonly VITE_API_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
