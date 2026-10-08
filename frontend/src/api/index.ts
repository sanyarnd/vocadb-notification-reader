import { createApi, createHttpClient } from "@/api/client";

let onUnauthorized: () => void = () => {};

/** Registers a callback invoked when the backend rejects the session. */
export function setUnauthorizedHandler(handler: () => void): void {
  onUnauthorized = handler;
}

export const api = createApi(
  createHttpClient({
    baseURL: import.meta.env.VITE_API_URL ?? "",
    onUnauthorized: () => onUnauthorized()
  })
);
