import { StorageSerializers, useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";
import { computed } from "vue";

import { api } from "@/api";
import type { AccountData, AuthenticationPayload, Database } from "@/api/dto";

export const useAccountStore = defineStore("account", () => {
  const token = useLocalStorage<string | null>("account.token", null, {
    serializer: StorageSerializers.string
  });
  const database = useLocalStorage<Database>("account.database", "VocaDb");
  const accountData = useLocalStorage<AccountData | null>("account.data", null, {
    serializer: StorageSerializers.object
  });

  const isAuthenticated = computed(() => token.value !== null && token.value !== "");

  async function login(payload: AuthenticationPayload): Promise<void> {
    const response = await api.authenticate(payload);
    token.value = response.token;
    database.value = payload.database;
    accountData.value = await api.accountData();
  }

  /** Forgets the session locally, e.g. when the backend reports it as expired. */
  function logout(): void {
    token.value = null;
    accountData.value = null;
  }

  /** Terminates the session on the backend and forgets it locally. */
  async function signOut(): Promise<void> {
    try {
      if (isAuthenticated.value) await api.logout();
    } catch {
      // The session is forgotten locally anyway.
    } finally {
      logout();
    }
  }

  return { token, database, accountData, isAuthenticated, login, logout, signOut };
});
