import { axios } from "@/backend/axios";
import { AccountData, AuthenticationPayload, NotificationsDTO } from "@/backend/dto";
import { PreferredLanguage } from "@/plugins/store/user-module";
import {AxiosResponse} from "axios";

export const api = {
  async authenticate(payload: AuthenticationPayload): Promise<void> {
    const encPass = encodeURIComponent(payload.password);
    const encName = encodeURIComponent(payload.username);
    const data = `username=${encName}&password=${encPass}`;

    return axios.post("/api/login/authentication", data, {
      headers: { "Content-Type": "application/x-www-form-urlencoded" }
    });
  },

  async accountData(): Promise<AccountData> {
    return axios.get<AccountData>("/api/v1/account").then(value => value.data);
  },

  async logout(): Promise<void> {
    return axios.post("/api/logout");
  },

  getNotifications(
    maxResults: number,
    startOffset: number,
    preferredLanguage: PreferredLanguage
  ): Promise<AxiosResponse<NotificationsDTO>> {
    return axios.get<NotificationsDTO>("/api/v1/notifications", {
      params: {
        startOffset: startOffset,
        maxResults: maxResults,
        languagePreference: preferredLanguage
      }
    });
  },

  async deleteNotifications(notificationIds: Array<number>): Promise<void> {
    return axios.delete(`/api/v1/notifications?notificationIds=${notificationIds}`);
  }
};
