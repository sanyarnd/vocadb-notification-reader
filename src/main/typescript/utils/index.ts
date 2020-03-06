export function stringToColor(str: string): string {
  function range(hash: number, min: number, max: number) {
    const diff = max - min;
    const x = ((hash % diff) + diff) % diff;
    return x + min;
  }

  function toHsl(
    hash: number,
    hue: [number, number] = [0, 360],
    sat: [number, number] = [60, 100],
    lit: [number, number] = [35, 50]
  ): string {
    if (hash === 0) return "0";

    const h = range(hash, hue[0], hue[1]);
    const s = range(hash, sat[0], sat[1]);
    const l = range(hash, lit[0], lit[1]);
    return `hsl(${h}, ${s}%, ${l}%)`;
  }

  function toHash(str: string): number {
    let hash = 0;
    if (str.length == 0) return hash;
    for (let i = 0; i < str.length; ++i) {
      hash = str.charCodeAt(i) + ((hash << 5) - hash);
      hash = hash & hash;
    }
    return hash;
  }

  return toHsl(toHash(str));
}
