import { tick } from "svelte";
import { loadCatalog } from "./api.js";
import { buildShelves, SHELF_ALL } from "./shelves.js";
import { nextSortId, sortGames } from "./sorts.js";
import { reportAfterPaint } from "./timing.js";

const STATUS_LOADING = "loading";
const STATUS_READY = "ready";
const STATUS_FAILED = "failed";
const DEFAULT_SORT = "name";
const LABEL_GRID_PAINTED = "grid-painted";

class CatalogState {
  status = $state(STATUS_LOADING);
  error = $state("");
  games = $state([]);
  collections = $state([]);
  shelfId = $state(SHELF_ALL);
  sortId = $state(DEFAULT_SORT);
  focusIndex = $state(0);

  shelves = $derived(buildShelves(this.collections, this.games));
  activeShelf = $derived(this.shelves.find((shelf) => shelf.id === this.shelfId) ?? this.shelves[0]);
  visibleGames = $derived(sortGames(this.activeShelf?.games ?? [], this.sortId));

  async load() {
    try {
      const catalog = await loadCatalog();
      this.games = catalog.games;
      this.collections = catalog.collections;
      this.status = STATUS_READY;
      tick().then(() => reportAfterPaint(LABEL_GRID_PAINTED));
    } catch (error) {
      this.error = String(error);
      this.status = STATUS_FAILED;
    }
  }

  selectShelf(id) {
    this.shelfId = id;
    this.focusIndex = 0;
  }

  stepShelf(direction) {
    const index = this.shelves.findIndex((shelf) => shelf.id === this.activeShelf.id);
    const count = this.shelves.length;
    this.selectShelf(this.shelves[(index + direction + count) % count].id);
  }

  cycleSort() {
    this.sortId = nextSortId(this.sortId);
    this.focusIndex = 0;
  }
}

export const catalog = new CatalogState();
