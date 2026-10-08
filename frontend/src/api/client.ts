import axios, { type AxiosInstance } from "axios";

import type {
  AccessToken,
  AccountData,
  AuthenticationPayload,
  NotificationsResponse,
  RequestLanguage
} from "@/api/dto";

export interface ApiClientOptions {
  baseURL: string;
  /** Returns the current access token, if any. */
  getToken: () => string | null;
  /** Called when an authenticated request is rejected with 401. */
  onUnauthorized: () => void;
}

export function createHttpClient(options: ApiClientOptions): AxiosInstance {
  const http = axios.create({
    baseURL: options.baseURL,
    timeout: 45_000,
    // Lets an authenticating reverse proxy (e.g. Authelia) see its session cookie.
    withCredentials: true
  });

  http.interceptors.request.use(config => {
    const token = options.getToken();
    if (token !== null) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  });

  http.interceptors.response.use(undefined, error => {
    const isSessionRequest = [LOGIN_URL, LOGOUT_URL].includes(error?.config?.url);
    if (axios.isAxiosError(error) && error.response?.status === 401 && !isSessionRequest) {
      options.onUnauthorized();
    }
    return Promise.reject(error);
  });

  return http;
}

const LOGIN_URL = "/api/login";
const LOGOUT_URL = "/api/logout";

export function createApi(http: AxiosInstance) {
  return {
    async authenticate(payload: AuthenticationPayload): Promise<AccessToken> {
      return (await http.post<AccessToken>(LOGIN_URL, payload)).data;
    },

    async logout(): Promise<void> {
      await http.post(LOGOUT_URL);
    },

    async accountData(): Promise<AccountData> {
      return (await http.post<AccountData>("/api/users/current")).data;
    },

    async notifications(
      maxResults: number,
      startOffset: number,
      language: RequestLanguage
    ): Promise<NotificationsResponse> {
      const payload = { startOffset, maxResults, language };
      return (await http.post<NotificationsResponse>("/api/notifications/fetch", payload)).data;
    },

    async deleteNotifications(ids: number[]): Promise<void> {
      await http.post("/api/notifications/delete", { ids });
    }
  };
}

export type Api = ReturnType<typeof createApi>;

export function isUnauthorized(error: unknown): boolean {
  return axios.isAxiosError(error) && error.response?.status === 401;
}
