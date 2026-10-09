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

  it("requests notifications of a type with paging, language and search", async () => {
    const { api, mock } = setup();
    const query = {
      type: "artist",
      offset: 50,
      limit: 25,
      language: "Romaji",
      search: "miku"
    } as const;
    const response = {
      totalCount: 0,
      notifications: [],
      counts: { song: 0, artist: 0, album: 0, event: 0, report: 0, unknown: 0 }
    };
    mock.onGet("/api/notifications").reply(config => {
      expect(config.params).toEqual(query);
      return [200, response];
    });

    await expect(api.notifications(query)).resolves.toEqual(response);
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
