import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { nextTick } from "vue";

import { api } from "@/api";
import { useAccountStore } from "@/stores/account";
import { useSettingsStore } from "@/stores/settings";

import { account } from "./fixtures";

vi.mock("@/api", () => ({
  api: {
    login: vi.fn(),
    logout: vi.fn(),
    me: vi.fn()
  }
}));

describe("account store", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("logs in and remembers the account", async () => {
    const touhou = account({ database: "TouhouDb" });
    vi.mocked(api.login).mockResolvedValue(touhou);

    const store = useAccountStore();
    expect(store.isAuthenticated).toBe(false);

    await store.login({ username: "miku", password: "pw", database: "TouhouDb" });
    await nextTick();

    expect(store.isAuthenticated).toBe(true);
    expect(store.account).toEqual(touhou);
    expect(store.database).toBe("TouhouDb");

    setActivePinia(createPinia());
    const restored = useAccountStore();
    expect(restored.isAuthenticated).toBe(true);
    expect(restored.account).toEqual(touhou);
    expect(restored.lastDatabase).toBe("TouhouDb");
  });

  it("stays logged out when login fails", async () => {
    vi.mocked(api.login).mockRejectedValue(new Error("401"));

    const store = useAccountStore();
    await expect(
      store.login({ username: "miku", password: "pw", database: "VocaDb" })
    ).rejects.toThrow();
    expect(store.isAuthenticated).toBe(false);
  });

  it("refreshes the account", async () => {
    vi.mocked(api.me).mockResolvedValue(account({ database: "UtaiteDb" }));
    const store = useAccountStore();
    store.account = account();

    await store.refresh();

    expect(store.database).toBe("UtaiteDb");
  });

  it("terminates the backend session on sign out", async () => {
    vi.mocked(api.logout).mockResolvedValue();
    const store = useAccountStore();
    store.account = account();

    await store.signOut();
    await nextTick();

    expect(api.logout).toHaveBeenCalledOnce();
    expect(store.isAuthenticated).toBe(false);
    expect(localStorage.getItem("account")).toBeNull();
  });

  it("forgets the session even if the backend fails", async () => {
    vi.mocked(api.logout).mockRejectedValue(new Error("Network Error"));
    const store = useAccountStore();
    store.account = account();

    await store.signOut();

    expect(store.isAuthenticated).toBe(false);
  });

  it("keeps the last database for the login page", () => {
    const store = useAccountStore();
    expect(store.database).toBe("VocaDb");
    store.lastDatabase = "UtaiteDb";
    expect(store.database).toBe("UtaiteDb");
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
