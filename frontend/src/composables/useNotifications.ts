import { computed, ref, watch } from "vue";

import { api } from "@/api";
import { isRateLimited } from "@/api/client";
import type { VocaDbNotification } from "@/api/dto";
import { useSettingsStore } from "@/stores/settings";

function errorKey(e: unknown): string {
  return isRateLimited(e) ? "$vuetify.tooManyRequests" : "$vuetify.connectionError";
}

/** Server-side paginated notifications list. */
export function useNotifications() {
  const settings = useSettingsStore();

  const notifications = ref<VocaDbNotification[]>([]);
  const totalCount = ref(0);
  const page = ref(1);
  const loading = ref(false);
  const deleting = ref(false);
  /** Translation key of the last error, if any. */
  const error = ref<string | null>(null);

  const pageCount = computed(() =>
    Math.max(1, Math.ceil(totalCount.value / settings.itemsPerPage))
  );

  async function load(targetPage: number): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      const response = await api.notifications(
        settings.itemsPerPage,
        settings.itemsPerPage * (targetPage - 1),
        settings.preferredLanguage
      );
      notifications.value = response.notifications;
      totalCount.value = response.totalCount;
      page.value = targetPage;
    } catch (e) {
      error.value = errorKey(e);
    } finally {
      loading.value = false;
    }
  }

  async function remove(ids: number[]): Promise<boolean> {
    if (ids.length === 0) return true;

    deleting.value = true;
    error.value = null;
    try {
      await api.deleteNotifications(ids);
      const before = notifications.value.length;
      notifications.value = notifications.value.filter(n => !ids.includes(n.id));
      totalCount.value = Math.max(0, totalCount.value - (before - notifications.value.length));
      return true;
    } catch (e) {
      error.value = errorKey(e);
      return false;
    } finally {
      deleting.value = false;
    }
  }

  watch(
    () => settings.preferredLanguage,
    () => load(page.value)
  );
  watch(
    () => settings.itemsPerPage,
    () => load(1)
  );

  return { notifications, totalCount, page, pageCount, loading, deleting, error, load, remove };
}
