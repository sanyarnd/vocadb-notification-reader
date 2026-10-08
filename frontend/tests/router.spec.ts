import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { createAppRouter } from "@/router";
import { useAccountStore } from "@/stores/account";

import { account } from "./fixtures";

vi.mock("@/views/HomeView.vue", () => ({ default: { template: "<div />" } }));
vi.mock("@/views/LoginView.vue", () => ({ default: { template: "<div />" } }));

describe("router", () => {
  beforeEach(() => setActivePinia(createPinia()));

  it("redirects anonymous users to login", async () => {
    const router = createAppRouter();
    await router.push("/");
    expect(router.currentRoute.value.name).toBe("login");
  });

  it("lets authenticated users in and away from login", async () => {
    useAccountStore().account = account();
    const router = createAppRouter();

    await router.push("/login");
    expect(router.currentRoute.value.name).toBe("home");

    await router.push("/");
    expect(router.currentRoute.value.name).toBe("home");
  });

  it("redirects unknown paths home", async () => {
    useAccountStore().account = account();
    const router = createAppRouter();
    await router.push("/does/not/exist");
    expect(router.currentRoute.value.name).toBe("home");
  });
});
