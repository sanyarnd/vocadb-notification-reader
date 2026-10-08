/** Formats an ISO date for display, e.g. "Fri, Dec 7, 2007". */
export function formatDate(value: string | null, locale: string): string | null {
  if (value === null) return null;
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return null;
  return date.toLocaleDateString(locale, {
    weekday: "short",
    day: "numeric",
    month: "short",
    year: "numeric"
  });
}
