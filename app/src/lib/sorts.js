const compareText = (a, b) => a.name.localeCompare(b.name, undefined, { sensitivity: "base" });
const descending = (pick) => (a, b) => (pick(b) ?? -1) - (pick(a) ?? -1) || compareText(a, b);
const installedSize = (game) => game.copies.reduce((total, copy) => total + copy.size_on_disk, 0);

export const SORTS = [
  { id: "name", compare: compareText },
  { id: "lastPlayed", compare: descending((game) => game.last_played_unix) },
  { id: "playtime", compare: descending((game) => game.playtime_minutes) },
  { id: "rating", compare: descending((game) => game.review_percentage) },
  { id: "release", compare: descending((game) => game.release_unix) },
  { id: "size", compare: descending(installedSize) },
];

export function nextSortId(currentId) {
  const index = SORTS.findIndex((sort) => sort.id === currentId);
  return SORTS[(index + 1) % SORTS.length].id;
}

export function sortGames(games, sortId) {
  const sort = SORTS.find((entry) => entry.id === sortId) ?? SORTS[0];
  return [...games].sort(sort.compare);
}
