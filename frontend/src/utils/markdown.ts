const markdownLink = /\[(.*)\]\((.*?)\)/;

/** Replaces the first markdown link with its description. */
export function removeMarkdown(markdown: string): string {
  return markdown.replace(markdownLink, "$1");
}

/** Extracts the URL of the first markdown link. */
export function extractUrlFromMarkdown(markdown: string): string | null {
  return markdownLink.exec(markdown)?.[2] ?? null;
}
