import originalAxios from "axios";
import * as AxiosLogger from "axios-logger";
import { authenticationExpireHandler } from "@/backend/authenticationExpireHandler";

export const axios = originalAxios.create({
  timeout: 45 * 1000 // 45s
});

axios.interceptors.request.use(
  value => value,
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

if (process.env.NODE_ENV === "development") {
  axios.interceptors.request.use(AxiosLogger.requestLogger, AxiosLogger.errorLogger);
  axios.interceptors.response.use(AxiosLogger.responseLogger, AxiosLogger.errorLogger);
}
