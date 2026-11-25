import strings from "./strings.json";

export const SHELF_ALL = "all";
export const SHELF_INSTALLED = "installed";
const KIND_HIDDEN = "hidden";

export function buildShelves(collections, games) {
  const visible = games.filter((game) => !game.hidden);
  const collected = collections
    .filter((collection) => collection.kind !== KIND_HIDDEN && collection.app_ids.length > 0)
    .map((collection) => ({
      id: collection.id,
      label: collection.name,
      games: visible.filter((game) => game.collection_ids.includes(collection.id)),
    }));
  return [
    { id: SHELF_ALL, label: strings.shelfAll, games: visible },
    { id: SHELF_INSTALLED, label: strings.shelfInstalled, games: visible.filter((g) => g.copies.length > 0) },
    ...collected,
  ];
}
