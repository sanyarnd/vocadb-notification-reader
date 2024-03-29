import originalAxios from "axios";
import { authenticationExpireHandler } from "@/backend/authenticationExpireHandler";

import { store } from "@/plugins/store";
import { accountModule } from "@/plugins/store/account-module";

export const axios = originalAxios.create({
  timeout: 45 * 1000,
  baseURL:
    process.env.NODE_ENV !== "production"
      ? "http://localhost:8080"
      : "https://api.vocadb-notification-reader.handystuff.net"
});

axios.interceptors.request.use(
  value => {
    const userSettings = accountModule.context(store);
    if (value.headers != undefined) {
      value.headers["Authorization"] = `Bearer ${userSettings.state.accessToken}`;
    }
    return value;
  },
  error => Promise.reject(error)
);
axios.interceptors.response.use(
  value => value,
  error => {
    const response = error.response;
    const status = error.status || (response ? response.status : 0);
    if (status === 401) authenticationExpireHandler();
    return Promise.reject(error);
  }
);
