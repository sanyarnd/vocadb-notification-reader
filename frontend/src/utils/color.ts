/** Deterministically maps a string to a saturated HSL color. */
export function stringToColor(str: string): string {
  const hash = toHash(str);
  if (hash === 0) return "hsl(0, 0%, 50%)";

  const h = range(hash, 0, 360);
  const s = range(hash, 60, 100);
  const l = range(hash, 35, 50);
  return `hsl(${h}, ${s}%, ${l}%)`;
}

function range(hash: number, min: number, max: number): number {
  const diff = max - min;
  return (((hash % diff) + diff) % diff) + min;
}

function toHash(str: string): number {
  let hash = 0;
  for (let i = 0; i < str.length; ++i) {
    hash = (str.charCodeAt(i) + ((hash << 5) - hash)) | 0;
  }
  return hash;
}
