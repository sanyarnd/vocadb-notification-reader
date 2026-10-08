import { StorageSerializers, useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";
import { computed } from "vue";

import { api } from "@/api";
import type { Account, Database, LoginRequest } from "@/api/dto";

// Leftovers of the bearer token based sessions.
for (const key of ["account.token", "account.data"]) localStorage.removeItem(key);

/**
 * The session itself is an httpOnly cookie the frontend can't see; the account is kept
 * locally to render the UI right away and is refreshed from the backend.
 */
export const useAccountStore = defineStore("account", () => {
  const account = useLocalStorage<Account | null>("account", null, {
    serializer: StorageSerializers.object
  });
  /** Database chosen on the last login, preselected on the login page. */
  const lastDatabase = useLocalStorage<Database>("account.lastDatabase", "VocaDb");

  const isAuthenticated = computed(() => account.value !== null);
  const database = computed(() => account.value?.database ?? lastDatabase.value);

  async function login(payload: LoginRequest): Promise<void> {
    account.value = await api.login(payload);
    lastDatabase.value = payload.database;
  }

  /** Re-validates the session; a 401 ends up in [logout] through the API client. */
  async function refresh(): Promise<void> {
    account.value = await api.me();
  }

  /** Forgets the session locally, e.g. when the backend reports it as ended. */
  function logout(): void {
    account.value = null;
  }

  /** Terminates the session on the backend and forgets it locally. */
  async function signOut(): Promise<void> {
    try {
      await api.logout();
    } catch {
      // The session is forgotten locally anyway.
    } finally {
      logout();
    }
  }

  return {
    account,
    lastDatabase,
    database,
    isAuthenticated,
    login,
    refresh,
    logout,
    signOut
  };
});
