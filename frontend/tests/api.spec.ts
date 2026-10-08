import MockAdapter from "axios-mock-adapter";
import { describe, expect, it, vi } from "vitest";

import { createApi, createHttpClient, isUnauthorized } from "@/api/client";

function setup(token: string | null = "secret") {
  const onUnauthorized = vi.fn();
  const http = createHttpClient({ baseURL: "", getToken: () => token, onUnauthorized });
  const mock = new MockAdapter(http);
  return { api: createApi(http), mock, onUnauthorized };
}

describe("api client", () => {
  it("sends the bearer token", async () => {
    const { api, mock } = setup();
    mock.onPost("/api/users/current").reply(config => {
      expect(config.headers?.Authorization).toBe("Bearer secret");
      return [200, { id: 1, name: "miku" }];
    });

    await expect(api.accountData()).resolves.toMatchObject({ id: 1, name: "miku" });
  });

  it("does not send a header without a token", async () => {
    const { api, mock } = setup(null);
    mock.onPost("/api/login").reply(config => {
      expect(config.headers?.Authorization).toBeUndefined();
      expect(JSON.parse(config.data)).toEqual({
        username: "miku",
        password: "pw",
        database: "VocaDb"
      });
      return [200, { token: "t" }];
    });

    await expect(
      api.authenticate({ username: "miku", password: "pw", database: "VocaDb" })
    ).resolves.toEqual({ token: "t" });
  });

  it("requests notifications with paging and language", async () => {
    const { api, mock } = setup();
    mock.onPost("/api/notifications/fetch").reply(config => {
      expect(JSON.parse(config.data)).toEqual({
        startOffset: 50,
        maxResults: 25,
        language: "Romaji"
      });
      return [200, { totalCount: 0, notifications: [] }];
    });

    await expect(api.notifications(25, 50, "Romaji")).resolves.toEqual({
      totalCount: 0,
      notifications: []
    });
  });

  it("deletes notifications by id", async () => {
    const { api, mock } = setup();
    mock.onPost("/api/notifications/delete", { ids: [1, 2] }).reply(200, null);

    await expect(api.deleteNotifications([1, 2])).resolves.toBeUndefined();
  });

  it("reports an expired session", async () => {
    const { api, mock, onUnauthorized } = setup();
    mock.onPost("/api/users/current").reply(401, { code: 401 });

    const error = await api.accountData().catch(e => e);
    expect(isUnauthorized(error)).toBe(true);
    expect(onUnauthorized).toHaveBeenCalledOnce();
  });

  it("does not treat failed login as an expired session", async () => {
    const { api, mock, onUnauthorized } = setup(null);
    mock.onPost("/api/login").reply(401);

    await expect(
      api.authenticate({ username: "miku", password: "wrong", database: "VocaDb" })
    ).rejects.toThrow();
    expect(onUnauthorized).not.toHaveBeenCalled();
  });

  it("logs out without triggering the expired session handler", async () => {
    const { api, mock, onUnauthorized } = setup();
    mock.onPost("/api/logout").replyOnce(200, null).onPost("/api/logout").reply(401);

    await expect(api.logout()).resolves.toBeUndefined();
    await expect(api.logout()).rejects.toThrow();
    expect(onUnauthorized).not.toHaveBeenCalled();
  });

  it("ignores other errors", async () => {
    const { api, mock, onUnauthorized } = setup();
    mock.onPost("/api/users/current").reply(502);

    const error = await api.accountData().catch(e => e);
    expect(isUnauthorized(error)).toBe(false);
    expect(onUnauthorized).not.toHaveBeenCalled();
  });
});
