import MockAdapter from "axios-mock-adapter";
import { describe, expect, it, vi } from "vitest";

import { createApi, createHttpClient, isUnauthorized } from "@/api/client";

import { account } from "./fixtures";

function setup() {
  const onUnauthorized = vi.fn();
  const http = createHttpClient({ baseURL: "", onUnauthorized });
  const mock = new MockAdapter(http);
  return { api: createApi(http), http, mock, onUnauthorized };
}

describe("api client", () => {
  it("sends credentials with every request", () => {
    expect(setup().http.defaults.withCredentials).toBe(true);
  });

  it("logs in", async () => {
    const { api, mock } = setup();
    mock.onPost("/api/session").reply(config => {
      expect(JSON.parse(config.data)).toEqual({
        username: "miku",
        password: "pw",
        database: "VocaDb"
      });
      return [200, account()];
    });

    await expect(
      api.login({ username: "miku", password: "pw", database: "VocaDb" })
    ).resolves.toEqual(account());
  });

  it("logs out", async () => {
    const { api, mock } = setup();
    mock.onDelete("/api/session").reply(200, null);

    await expect(api.logout()).resolves.toBeUndefined();
  });

  it("loads the account", async () => {
    const { api, mock } = setup();
    mock.onGet("/api/me").reply(200, account());

    await expect(api.me()).resolves.toEqual(account());
  });

  it("requests notifications with paging and language", async () => {
    const { api, mock } = setup();
    mock.onGet("/api/notifications").reply(config => {
      expect(config.params).toEqual({ offset: 50, limit: 25, language: "Romaji" });
      return [200, { totalCount: 0, notifications: [] }];
    });

    await expect(api.notifications(25, 50, "Romaji")).resolves.toEqual({
      totalCount: 0,
      notifications: []
    });
  });

  it("deletes notifications by id", async () => {
    const { api, mock } = setup();
    mock.onDelete("/api/notifications").reply(config => {
      expect(JSON.parse(config.data)).toEqual({ ids: [1, 2] });
      return [200, null];
    });

    await expect(api.deleteNotifications([1, 2])).resolves.toBeUndefined();
  });

  it("reports an ended session", async () => {
    const { api, mock, onUnauthorized } = setup();
    mock.onGet("/api/me").reply(401, { code: 401 });

    const error: unknown = await api.me().catch((e: unknown) => e);
    expect(isUnauthorized(error)).toBe(true);
    expect(onUnauthorized).toHaveBeenCalledOnce();
  });

  it("does not treat session requests as an ended session", async () => {
    const { api, mock, onUnauthorized } = setup();
    mock.onPost("/api/session").reply(401).onDelete("/api/session").reply(401);

    await expect(
      api.login({ username: "miku", password: "wrong", database: "VocaDb" })
    ).rejects.toThrow();
    await expect(api.logout()).rejects.toThrow();
    expect(onUnauthorized).not.toHaveBeenCalled();
  });

  it("ignores other errors", async () => {
    const { api, mock, onUnauthorized } = setup();
    mock.onGet("/api/me").reply(502);

    const error: unknown = await api.me().catch((e: unknown) => e);
    expect(isUnauthorized(error)).toBe(false);
    expect(onUnauthorized).not.toHaveBeenCalled();
  });
});
