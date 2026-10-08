import { computed, ref, watch } from "vue";

import { api } from "@/api";
import type { VocaDbNotification } from "@/api/dto";
import { useSettingsStore } from "@/stores/settings";

/** Server-side paginated notifications list. */
export function useNotifications() {
  const settings = useSettingsStore();

  const notifications = ref<VocaDbNotification[]>([]);
  const totalCount = ref(0);
  const page = ref(1);
  const loading = ref(false);
  const deleting = ref(false);
  const failed = ref(false);

  const pageCount = computed(() =>
    Math.max(1, Math.ceil(totalCount.value / settings.itemsPerPage))
  );

  async function load(targetPage: number): Promise<void> {
    loading.value = true;
    failed.value = false;
    try {
      const response = await api.notifications(
        settings.itemsPerPage,
        settings.itemsPerPage * (targetPage - 1),
        settings.preferredLanguage
      );
      notifications.value = response.notifications;
      totalCount.value = response.totalCount;
      page.value = targetPage;
    } catch {
      failed.value = true;
    } finally {
      loading.value = false;
    }
  }

  async function remove(ids: number[]): Promise<boolean> {
    if (ids.length === 0) return true;

    deleting.value = true;
    failed.value = false;
    try {
      await api.deleteNotifications(ids);
      const before = notifications.value.length;
      notifications.value = notifications.value.filter(n => !ids.includes(n.id));
      totalCount.value = Math.max(0, totalCount.value - (before - notifications.value.length));
      return true;
    } catch {
      failed.value = true;
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

  return { notifications, totalCount, page, pageCount, loading, deleting, failed, load, remove };
}
