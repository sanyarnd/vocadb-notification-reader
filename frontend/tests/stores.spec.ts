import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { nextTick } from "vue";

import { api } from "@/api";
import { useAccountStore } from "@/stores/account";
import { useSettingsStore } from "@/stores/settings";

vi.mock("@/api", () => ({
  api: {
    authenticate: vi.fn(),
    accountData: vi.fn()
  }
}));

const account = {
  id: 1,
  name: "miku",
  active: true,
  memberSince: "2020-01-01",
  verifiedArtist: false,
  groupId: "Regular",
  mainPicture: null
};

describe("account store", () => {
  beforeEach(() => setActivePinia(createPinia()));

  it("logs in and persists the session", async () => {
    vi.mocked(api.authenticate).mockResolvedValue({ token: "token" });
    vi.mocked(api.accountData).mockResolvedValue(account);

    const store = useAccountStore();
    expect(store.isAuthenticated).toBe(false);

    await store.login({ username: "miku", password: "pw", database: "TouhouDb" });

    expect(store.isAuthenticated).toBe(true);
    expect(store.token).toBe("token");
    expect(store.database).toBe("TouhouDb");
    expect(store.accountData).toEqual(account);
    expect(localStorage.getItem("account.token")).toBe("token");

    setActivePinia(createPinia());
    const restored = useAccountStore();
    expect(restored.isAuthenticated).toBe(true);
    expect(restored.database).toBe("TouhouDb");
    expect(restored.accountData).toEqual(account);
  });

  it("stays logged out when login fails", async () => {
    vi.mocked(api.authenticate).mockRejectedValue(new Error("401"));

    const store = useAccountStore();
    await expect(
      store.login({ username: "miku", password: "pw", database: "VocaDb" })
    ).rejects.toThrow();
    expect(store.isAuthenticated).toBe(false);
  });

  it("logs out", async () => {
    vi.mocked(api.authenticate).mockResolvedValue({ token: "token" });
    vi.mocked(api.accountData).mockResolvedValue(account);

    const store = useAccountStore();
    await store.login({ username: "miku", password: "pw", database: "VocaDb" });
    store.logout();
    await nextTick();

    expect(store.isAuthenticated).toBe(false);
    expect(store.accountData).toBeNull();
    expect(localStorage.getItem("account.token")).toBeNull();
  });
});

describe("settings store", () => {
  beforeEach(() => setActivePinia(createPinia()));

  it("has defaults", () => {
    const settings = useSettingsStore();
    expect(settings.theme).toBe("light");
    expect(settings.locale).toBe("en");
    expect(settings.itemsPerPage).toBe(25);
    expect(settings.preferredLanguage).toBe("Default");
    expect(settings.preferredPvService).toBe("NicoNicoDouga");
  });

  it("persists changes", async () => {
    const settings = useSettingsStore();
    settings.toggleTheme();
    settings.locale = "ru";
    settings.itemsPerPage = 50;
    await nextTick();

    setActivePinia(createPinia());
    const restored = useSettingsStore();
    expect(restored.theme).toBe("dark");
    expect(restored.locale).toBe("ru");
    expect(restored.itemsPerPage).toBe(50);

    restored.toggleTheme();
    expect(restored.theme).toBe("light");
  });
});
