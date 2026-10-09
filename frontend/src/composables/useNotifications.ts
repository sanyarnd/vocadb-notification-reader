import { refDebounced } from "@vueuse/core";
import { computed, ref, watch } from "vue";

import { api } from "@/api";
import { isRateLimited } from "@/api/client";
import type { KindCounts, NotificationKind, VocaDbNotification } from "@/api/dto";
import { useSettingsStore } from "@/stores/settings";

function errorKey(e: unknown): string {
  return isRateLimited(e) ? "$vuetify.tooManyRequests" : "$vuetify.connectionError";
}

const emptyCounts: KindCounts = { song: 0, artist: 0, album: 0, event: 0, report: 0, unknown: 0 };

/** Notifications of one kind, filtered and paginated by the backend over the whole inbox. */
export function useNotifications() {
  const settings = useSettingsStore();

  const kind = ref<NotificationKind>("song");
  const search = ref<string | null>("");
  const debouncedSearch = refDebounced(search, 400);

  const notifications = ref<VocaDbNotification[]>([]);
  const totalCount = ref(0);
  const counts = ref<KindCounts>(emptyCounts);
  const page = ref(1);
  const loading = ref(false);
  const deleting = ref(false);
  /** Translation key of the last error, if any. */
  const error = ref<string | null>(null);

  const pageCount = computed(() =>
    Math.max(1, Math.ceil(totalCount.value / settings.itemsPerPage))
  );

  /** Responses to outdated requests (e.g. while typing) are dropped. */
  let latestRequest = 0;

  async function load(targetPage: number): Promise<void> {
    const request = ++latestRequest;
    loading.value = true;
    error.value = null;
    try {
      const response = await api.notifications({
        type: kind.value,
        offset: settings.itemsPerPage * (targetPage - 1),
        limit: settings.itemsPerPage,
        language: settings.preferredLanguage,
        search: debouncedSearch.value?.trim() || undefined
      });
      if (request !== latestRequest) return;
      notifications.value = response.notifications;
      totalCount.value = response.totalCount;
      counts.value = response.counts;
      page.value = targetPage;
    } catch (e) {
      if (request === latestRequest) error.value = errorKey(e);
    } finally {
      if (request === latestRequest) loading.value = false;
    }
  }

  async function remove(ids: number[]): Promise<boolean> {
    if (ids.length === 0) return true;

    deleting.value = true;
    error.value = null;
    try {
      await api.deleteNotifications(ids);
    } catch (e) {
      error.value = errorKey(e);
      return false;
    } finally {
      deleting.value = false;
    }

    // Counts and the following notifications shift, reload the page (or the last one left).
    const remaining = Math.max(0, totalCount.value - ids.length);
    const lastPage = Math.max(1, Math.ceil(remaining / settings.itemsPerPage));
    await load(Math.min(page.value, lastPage));
    return true;
  }

  watch(
    () => settings.preferredLanguage,
    () => load(page.value)
  );
  watch([kind, debouncedSearch, () => settings.itemsPerPage], () => load(1));

  return {
    kind,
    search,
    notifications,
    totalCount,
    counts,
    page,
    pageCount,
    loading,
    deleting,
    error,
    load,
    remove
  };
}
