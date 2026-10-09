import { flushPromises } from "@vue/test-utils";
import { AxiosError, AxiosHeaders } from "axios";
import { describe, expect, it, vi } from "vitest";
import { nextTick } from "vue";

import { api } from "@/api";
import NotificationTable from "@/components/NotificationTable.vue";
import SongNotificationPopup from "@/components/SongNotificationPopup.vue";
import { useAccountStore } from "@/stores/account";
import { useSettingsStore } from "@/stores/settings";
import LoginView from "@/views/LoginView.vue";

import { account, artistNotification, pv, songNotification } from "./fixtures";
import { mountWithPlugins } from "./mount";

vi.mock("@/api", () => ({
  api: {
    login: vi.fn(),
    notifications: vi.fn(),
    deleteNotifications: vi.fn()
  }
}));

function httpError(status: number): AxiosError {
  const config = { headers: new AxiosHeaders() };
  return new AxiosError("Request failed", "ERR_BAD_REQUEST", config, null, {
    status,
    statusText: "",
    data: {},
    headers: {},
    config
  });
}

async function fillLoginForm(wrapper: ReturnType<typeof mountWithPlugins>["wrapper"]) {
  await wrapper.find("input[name=username]").setValue("miku");
  await wrapper.find("input[name=password]").setValue("secret");
  await flushPromises();
}

describe("LoginView", () => {
  it("logs in and navigates home", async () => {
    vi.mocked(api.login).mockResolvedValue(account());
    const { wrapper, router } = mountWithPlugins(LoginView);
    const push = vi.spyOn(router, "push");

    expect(wrapper.find("button[type=submit]").text()).toBe("Login with VocaDB");
    await fillLoginForm(wrapper);
    await wrapper.find("form").trigger("submit");
    await flushPromises();

    expect(api.login).toHaveBeenCalledWith({
      username: "miku",
      password: "secret",
      database: "VocaDb"
    });
    expect(useAccountStore().isAuthenticated).toBe(true);
    expect(push).toHaveBeenCalledWith({ name: "home" });
    wrapper.unmount();
  });

  it("shows an error for bad credentials", async () => {
    vi.mocked(api.login).mockRejectedValue(httpError(401));
    const { wrapper } = mountWithPlugins(LoginView);

    await fillLoginForm(wrapper);
    await wrapper.find("form").trigger("submit");
    await flushPromises();

    expect(wrapper.text()).toContain("Incorrect username or password");
    expect(useAccountStore().isAuthenticated).toBe(false);
    wrapper.unmount();
  });

  it("asks to wait when rate limited", async () => {
    vi.mocked(api.login).mockRejectedValue(httpError(429));
    const { wrapper } = mountWithPlugins(LoginView);

    await fillLoginForm(wrapper);
    await wrapper.find("form").trigger("submit");
    await flushPromises();

    expect(wrapper.text()).toContain("Too many attempts, try again later");
    wrapper.unmount();
  });

  it("shows a connection error otherwise", async () => {
    vi.mocked(api.login).mockRejectedValue(new Error("Network Error"));
    const { wrapper } = mountWithPlugins(LoginView);

    await fillLoginForm(wrapper);
    await wrapper.find("form").trigger("submit");
    await flushPromises();

    expect(wrapper.text()).toContain("Connection error");
    wrapper.unmount();
  });

  it("does not submit an empty form", async () => {
    const { wrapper } = mountWithPlugins(LoginView);
    await flushPromises();

    await wrapper.find("form").trigger("submit");
    await flushPromises();

    expect(api.login).not.toHaveBeenCalled();
    wrapper.unmount();
  });
});

describe("NotificationTable", () => {
  const song = songNotification({ id: 1, title: "Melt" });
  const otherSong = songNotification({
    id: 3,
    title: "World is Mine",
    tags: [{ id: 2, name: "pop", count: 1, categoryName: null }]
  });
  const artist = artistNotification(2, "[Miku](https://vocadb.net/Ar/1)");

  async function mountTable() {
    vi.mocked(api.notifications).mockResolvedValue({
      totalCount: 60,
      notifications: [song, artist, otherSong]
    });
    const result = mountWithPlugins(NotificationTable);
    await flushPromises();
    return result;
  }

  function rows(wrapper: Awaited<ReturnType<typeof mountTable>>["wrapper"]) {
    return wrapper.findAll("tbody tr").map(row => row.text());
  }

  it("loads the first page and shows songs", async () => {
    const { wrapper } = await mountTable();

    expect(api.notifications).toHaveBeenCalledWith(25, 0, "Default");
    expect(rows(wrapper)).toHaveLength(2);
    expect(rows(wrapper)[0]).toContain("Melt");
    expect(rows(wrapper)[1]).toContain("World is Mine");
    // 60 notifications with 25 per page
    expect(wrapper.findAll(".v-pagination__item")).toHaveLength(3);
    wrapper.unmount();
  });

  it("filters by search query", async () => {
    const { wrapper } = await mountTable();

    await wrapper.find(".v-text-field input").setValue("pop");
    expect(rows(wrapper)).toHaveLength(1);
    expect(rows(wrapper)[0]).toContain("World is Mine");
    wrapper.unmount();
  });

  it("switches tabs", async () => {
    const { wrapper } = await mountTable();

    const artistTab = wrapper.findAll(".v-tab").find(tab => tab.text().includes("Artist"))!;
    await artistTab.trigger("click");
    await flushPromises();

    expect(rows(wrapper)).toHaveLength(1);
    expect(rows(wrapper)[0]).toContain("New artist");
    expect(rows(wrapper)[0]).toContain("Miku");
    wrapper.unmount();
  });

  it("deletes selected notifications", async () => {
    vi.mocked(api.deleteNotifications).mockResolvedValue();
    const { wrapper } = await mountTable();

    await wrapper.find("tbody tr input[type=checkbox]").trigger("click");
    await flushPromises();
    const deleteButton = wrapper.findAll("button").find(b => b.text() === "Delete")!;
    expect(deleteButton.attributes("disabled")).toBeUndefined();
    await deleteButton.trigger("click");
    await flushPromises();

    expect(api.deleteNotifications).toHaveBeenCalledWith([1]);
    expect(rows(wrapper)).toHaveLength(1);
    expect(rows(wrapper)[0]).toContain("World is Mine");
    wrapper.unmount();
  });

  it("reloads when settings change", async () => {
    const { wrapper } = await mountTable();
    const settings = useSettingsStore();

    settings.preferredLanguage = "Romaji";
    await flushPromises();
    expect(api.notifications).toHaveBeenLastCalledWith(25, 0, "Romaji");

    settings.itemsPerPage = 50;
    await flushPromises();
    expect(api.notifications).toHaveBeenLastCalledWith(50, 0, "Romaji");
    wrapper.unmount();
  });

  it("reports rate limiting", async () => {
    vi.mocked(api.notifications).mockRejectedValue(httpError(429));
    const { wrapper } = mountWithPlugins(NotificationTable);
    await flushPromises();

    expect(document.body.textContent).toContain("Too many attempts, try again later");
    wrapper.unmount();
  });

  it("reports a failed request", async () => {
    vi.mocked(api.notifications).mockRejectedValue(new Error("Network Error"));
    const { wrapper } = mountWithPlugins(NotificationTable);
    await flushPromises();

    expect(document.body.textContent).toContain("Connection error");
    wrapper.unmount();
  });
});

describe("SongNotificationPopup", () => {
  it("opens the preferred service and links to the right database", async () => {
    const notification = songNotification({
      pvs: [
        pv({ id: 1, service: "NicoNicoDouga", pvId: "sm1" }),
        pv({ id: 2, service: "Youtube", pvId: "yt" }),
        pv({ id: 3, service: "Vimeo", pvId: "vm", disabled: true })
      ]
    });
    const { wrapper } = mountWithPlugins(SongNotificationPopup, { props: { notification: null } });
    useSettingsStore().preferredPvService = "Youtube";
    useAccountStore().account = account({ database: "TouhouDb" });

    await wrapper.setProps({ notification });
    await flushPromises();
    await nextTick();

    const tabs = [...document.querySelectorAll(".v-dialog .v-tab")];
    expect(tabs.map(tab => tab.textContent?.trim())).toEqual(["NicoNicoDouga", "Youtube"]);
    expect(document.querySelector(".v-dialog .v-tab--selected")?.textContent).toContain("Youtube");
    expect(document.querySelector(".v-dialog a[href]")?.getAttribute("href")).toBe(
      "https://touhoudb.com/S/100"
    );

    (document.querySelector(".v-dialog .v-btn.bg-error") as HTMLElement).click();
    expect(wrapper.emitted("delete")).toEqual([[1]]);
    wrapper.unmount();
  });
});
