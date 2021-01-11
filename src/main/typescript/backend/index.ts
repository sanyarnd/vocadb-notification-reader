import { axios } from "@/backend/axios";
import { AccountData, AuthenticationPayload, AccessToken, NotificationsDTO } from "@/backend/dto";
import { RequestLanguage } from "@/plugins/store/settings-module";
import { AxiosResponse } from "axios";

export const api = {
  async authenticate(payload: AuthenticationPayload): Promise<AccessToken> {
    return axios.post("/api/login", payload).then(value => value.data);
  },

  async accountData(): Promise<AccountData> {
    return axios.post<AccountData>("/api/users/current").then(value => value.data);
  },

  getNotifications(
    maxResults: number,
    startOffset: number,
    preferredLanguage: RequestLanguage
  ): Promise<AxiosResponse<NotificationsDTO>> {
    const payload = {
      startOffset: startOffset,
      maxResults: maxResults,
      language: preferredLanguage
    };

    return axios.post<NotificationsDTO>("/api/notifications/fetch", payload);
  },

  async deleteNotifications(notificationIds: Array<number>): Promise<void> {
    return axios.delete(`/api/v1/notifications?notificationIds=${notificationIds}`);
  }
};
