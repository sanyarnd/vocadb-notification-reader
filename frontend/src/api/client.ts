import axios, { type AxiosInstance } from "axios";

import type { Account, LoginRequest, NotificationsQuery, NotificationsResponse } from "@/api/dto";

export interface ApiClientOptions {
  baseURL: string;
  /** Called when the backend reports that the session has ended. */
  onUnauthorized: () => void;
}

const SESSION_URL = "/api/session";

export function createHttpClient(options: ApiClientOptions): AxiosInstance {
  const http = axios.create({
    baseURL: options.baseURL,
    timeout: 45_000,
    // The session lives in an httpOnly cookie of the API host.
    withCredentials: true
  });

  http.interceptors.response.use(undefined, error => {
    const isSessionRequest = error?.config?.url === SESSION_URL;
    if (axios.isAxiosError(error) && error.response?.status === 401 && !isSessionRequest) {
      options.onUnauthorized();
    }
    return Promise.reject(error);
  });

  return http;
}

export function createApi(http: AxiosInstance) {
  return {
    async login(payload: LoginRequest): Promise<Account> {
      return (await http.post<Account>(SESSION_URL, payload)).data;
    },

    async logout(): Promise<void> {
      await http.delete(SESSION_URL);
    },

    async me(): Promise<Account> {
      return (await http.get<Account>("/api/me")).data;
    },

    async notifications(params: NotificationsQuery): Promise<NotificationsResponse> {
      // Reading a large inbox for the first time takes a while.
      const config = { params, timeout: 180_000 };
      return (await http.get<NotificationsResponse>("/api/notifications", config)).data;
    },

    async deleteNotifications(ids: number[]): Promise<void> {
      await http.delete("/api/notifications", { data: { ids } });
    }
  };
}

export type Api = ReturnType<typeof createApi>;

export function isUnauthorized(error: unknown): boolean {
  return axios.isAxiosError(error) && error.response?.status === 401;
}

export function isRateLimited(error: unknown): boolean {
  return axios.isAxiosError(error) && error.response?.status === 429;
}
